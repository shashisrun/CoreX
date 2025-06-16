use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
mod deno_runtime;
mod route_manager;
use deno_runtime::CoreXDeno;
use route_manager::RouteManager;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let (tx, rx) = mpsc::channel::<(String, String, String, mpsc::Sender<String>)>();

    // Spawn Deno + RouteManager thread (no Arc/Mutex)
    thread::spawn(move || {
        let mut deno = CoreXDeno::new();
        let mut route_manager = RouteManager::new();
        route_manager.load_routes(&mut deno.runtime);
        for (method, path, req_json, resp_tx) in rx {
            let mut result = "Not found".to_string();
            println!("Loaded handlers: {:?}", route_manager.handlers.keys());
            println!("Looking for: ({:?}, {:?})", method, path);
            if let Some(handler) = route_manager.handlers.get(&(method.clone(), path.clone())) {
                let scope = &mut deno.runtime.handle_scope();
                let handler_fn = handler.open(scope);
                let global = scope.get_current_context().global(scope);
                let arg = deno_core::v8::String::new(scope, &req_json).unwrap();
                let req_obj = deno_core::v8::json::parse(scope, arg).unwrap();
                let res = handler_fn.call(scope, global.into(), &[req_obj]);
                if let Some(val) = res {
                    let val_str = val.to_string(scope).unwrap();
                    result = val_str.to_rust_string_lossy(scope);
                }
            }
            let _ = resp_tx.send(result);
        }
    });

    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let tx = tx.clone();
                async move {
                    let method = req.method().as_str().to_uppercase();
                    let path = req.uri().path().to_string();
                    let query: std::collections::HashMap<String, String> = req.uri().query().map(|q| {
                        q.split('&').filter_map(|kv| {
                            let mut split = kv.split('=');
                            if let (Some(k), Some(v)) = (split.next(), split.next()) {
                                Some((k.to_string(), v.to_string()))
                            } else { None }
                        }).collect()
                    }).unwrap_or_default();
                    let js_req = serde_json::json!({
                        "method": method,
                        "query": query,
                        "body": ""
                    });
                    let js_req_str = serde_json::to_string(&js_req).unwrap();
                    let (resp_tx, resp_rx) = mpsc::channel();
                    tx.send((method, path, js_req_str, resp_tx)).unwrap();
                    let result = resp_rx.recv().unwrap_or_else(|_| "Internal error".to_string());
                    Ok::<_, Infallible>(Response::new(Body::from(result)))
                }
            }))
        }
    });

    Server::bind(&addr).serve(make_svc).await.unwrap();
}