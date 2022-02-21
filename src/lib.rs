use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde_json::{Value};
use std::collections::HashMap;
use log::trace;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HeaderReplaceRootContext{
            headers: HashMap::new()
        })
    });
}

struct HeaderReplaceFilter{
    headers: HashMap<String, String>
}

impl Context for HeaderReplaceFilter {}

impl HttpContext for HeaderReplaceFilter {

    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {

        for (name, value) in &self.get_http_request_headers() {
            trace!("{}: {}", name, value);
            if name.eq("custom-header") {

                let config_header = &self.headers.get("my-key");

                self.add_http_request_header("Custom-WASM-Header", value);
                self.add_http_request_header("Custom-WASM-Config-Header", config_header.as_deref().unwrap_or(&"key not found".to_string()));
                self.set_http_request_header("custom-header", None);
            }
        }

        Action::Continue
    }
}

struct HeaderReplaceRootContext {
    headers: HashMap<String, String>
}

impl Context for HeaderReplaceRootContext {}

impl RootContext for HeaderReplaceRootContext {
    
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_configuration() {
            let config: Value = serde_json::from_slice(config_bytes.as_slice()).unwrap();
            let mut m = HashMap::new();
            for (key, value) in config.as_object().unwrap().iter() {
                m.insert(key.to_owned(), String::from(value.as_str().unwrap()));
            }
            self.headers = m;
        }
        true
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HeaderReplaceFilter{
            headers: self.headers.clone(),
        }))
    
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

}
