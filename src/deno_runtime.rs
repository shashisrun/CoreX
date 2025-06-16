use deno_core::{JsRuntime, RuntimeOptions, op2, Extension, anyhow::Result, serde_json::json};
use std::rc::Rc;

pub struct CoreXDeno {
    pub runtime: JsRuntime,
}

impl CoreXDeno {
    pub fn new() -> Self {
        let ext = Extension {
            name: "corex",
            ops: vec![op_log::decl()].into(),
            ..Default::default()
        };

        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        runtime.execute_script("<init>", r#"
            globalThis.greet = (name) => "Hello from Deno, " + name;
        "#).unwrap();

        Self { runtime }
    }

    pub fn call_greet(&mut self, name: &str) -> Result<String> {
        let script = format!("greet('{}')", name);
        let result = self.runtime.execute_script("<call>", script)?; // <-- pass script, not &script
        let scope = &mut self.runtime.handle_scope();
        let value = result.open(scope);
        let str_val = value.to_string(scope).unwrap();
        Ok(str_val.to_rust_string_lossy(scope))
    }
}

// Note the (fast) marker
#[op2(fast)]
fn op_log(#[string] msg: String) {
    println!("From Deno: {}", msg);
}