use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;
use deno_core::{JsRuntime, v8};

pub struct RouteManager {
    // (method, path) -> handler
    pub handlers: HashMap<(String, String), v8::Global<v8::Function>>,
    pub scripts: HashMap<(String, String), String>,
}

impl RouteManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            scripts: HashMap::new(),
        }
    }

    pub fn load_routes(&mut self, runtime: &mut JsRuntime) {
        self.handlers.clear();
        self.scripts.clear();
        // Recursively find all .ts files in routes/
        for entry in glob("src/routes/**/*.ts").unwrap().filter_map(Result::ok) {
            println!("Found route file: {:?}", entry);
            // e.g. routes/hello/get.ts -> method: GET, path: /hello
            let rel_path = entry.strip_prefix("src/routes").unwrap();
            let mut components: Vec<_> = rel_path.components().map(|c| c.as_os_str().to_str().unwrap()).collect();
            if let Some(file) = components.pop() {
                if let Some((method, _)) = file.split_once('.') {
                    let method = method.to_uppercase();
                    let path = format!("/{}", components.join("/"));
                    let script = fs::read_to_string(&entry).unwrap();
                    runtime.execute_script(
                        Box::leak(format!("{}:{}", method, path).into_boxed_str()),
                        script.clone()
                    ).unwrap();
                    let scope = &mut runtime.handle_scope();
                    let global = scope.get_current_context().global(scope);
                    let handler_key = v8::String::new(scope, "handler").unwrap();
                    let handler_val = global.get(scope, handler_key.into()).unwrap();
                    if handler_val.is_function() {
                        let handler = v8::Local::<v8::Function>::try_from(handler_val).unwrap();
                        self.handlers.insert((method.clone(), path.clone()), v8::Global::new(scope, handler));
                        self.scripts.insert((method, path), script);
                    }
                }
            }
        }
    }
}