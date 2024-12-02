#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        unsafe { tls_init() }
    }

    #[test]
    fn create_context() {
        let context = unsafe { tls_create_context(1, TLS_V12 as _) };
        assert!(!context.is_null());
        unsafe { tls_destroy_context(context) }
    }
}
