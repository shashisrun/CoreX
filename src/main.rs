use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
mod deno_runtime;
use deno_runtime::CoreXDeno;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // Channel for requests and responses
    let (tx, rx) = mpsc::channel::<(String, mpsc::Sender<String>)>();

    // Spawn Deno thread
    thread::spawn(move || {
        let mut deno = CoreXDeno::new();
        for (name, resp_tx) in rx {
            let result = deno.call_greet(&name).unwrap_or_else(|e| format!("Error: {}", e));
            let _ = resp_tx.send(result);
        }
    });

    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let tx = tx.clone();
                async move {
                    // Extract name from query, default to "CoreX"
                    let name = req.uri().query()
                        .and_then(|q| q.split('&').find_map(|kv| {
                            let mut split = kv.split('=');
                            if let (Some(k), Some(v)) = (split.next(), split.next()) {
                                if k == "name" { return Some(v.to_string()); }
                            }
                            None
                        }))
                        .unwrap_or_else(|| "CoreX".to_string());
                    // Channel for response
                    let (resp_tx, resp_rx) = mpsc::channel();
                    tx.send((name, resp_tx)).unwrap();
                    let result = resp_rx.recv().unwrap_or_else(|_| "Internal error".to_string());
                    Ok::<_, Infallible>(Response::new(Body::from(result)))
                }
            }))
        }
    });

    Server::bind(&addr).serve(make_svc).await.unwrap();
}