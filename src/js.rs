use anyhow::{anyhow, Result};
use rusty_v8 as v8;

pub struct V8Wrapper {
    isolate: v8::OwnedIsolate,
    context: Option<v8::Global<v8::Context>>,
}

impl V8Wrapper {
    pub fn new() -> Self {
        // Initialize V8.
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        // Create a new Isolate and make it the current one.
        let isolate = v8::Isolate::new(v8::CreateParams::default());

        V8Wrapper {
            isolate,
            context: None,
        }
    }

    pub fn execute_script(&mut self, source_code: &str) -> Result<String> {
        let handle_scope = &mut v8::HandleScope::new(&mut self.isolate);

        // Create context if it doesn't exist
        if self.context.is_none() {
            let context = v8::Context::new(handle_scope);
            self.context = Some(v8::Global::new(handle_scope, context));
        }

        // Get the persistent context
        let context = v8::Local::new(handle_scope, self.context.as_ref().unwrap());
        let mut scope = v8::ContextScope::new(handle_scope, context);

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(&mut scope, source_code)
            .ok_or(anyhow!("Failed to create V8 string"))?;
        // Compile the source code.
        let script = v8::Script::compile(&mut *scope, code, None)
            .ok_or(anyhow!("Failed to compile script"))?;
        // Run the script to get the result.
        let result = script
            .run(&mut *scope)
            .ok_or(anyhow!("Failed to run script"))?;
        // Convert the result to a string and return it.
        let result_str = result
            .to_string(&mut scope)
            .ok_or(anyhow!("Failed to convert result to string"))?;
        Ok(result_str.to_rust_string_lossy(&mut scope))
    }

    pub fn reset(&mut self) {
        self.context = None;
    }
}
