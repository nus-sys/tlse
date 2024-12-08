use std::{env::var, fs::read};

use criterion::{criterion_group, criterion_main, Criterion};
use tlse::{
    tls_accept, tls_buffer_clear, tls_client_connect, tls_consume_stream, tls_create_context,
    tls_destroy_context, tls_established, tls_export_context, tls_get_write_buffer,
    tls_import_context, tls_load_certificates, tls_load_private_key, tls_make_exportable,
    TLSContext, TLS_V12,
};

fn create_connection_context() -> *mut TLSContext {
    let cert_file = var("CERT_FILE").unwrap_or("./examples/cert/server.crt".into());
    let cert = read(cert_file).expect("can read certificate file");
    let key_file = var("KEY_FILE").unwrap_or("./examples/cert/server.key".into());
    let key = read(key_file).expect("can read key file");
    let server_context = unsafe { tls_create_context(1, TLS_V12 as _) };
    unsafe { tls_load_certificates(server_context, cert.as_ptr(), cert.len() as _) };
    unsafe { tls_load_private_key(server_context, key.as_ptr(), key.len() as _) };

    let client_context = unsafe { tls_create_context(0, TLS_V12 as _) };
    unsafe { tls_client_connect(client_context) };

    let context = unsafe { tls_accept(server_context) };
    unsafe { tls_make_exportable(context, 1) }
    while unsafe { tls_established(context) } == 0 {
        println!("establishing");
        let mut write_buf_len = 0;
        let write_buf = unsafe { tls_get_write_buffer(client_context, &raw mut write_buf_len) };
        println!("  client -> server {write_buf_len} bytes");
        if write_buf_len != 0 {
            unsafe { tls_consume_stream(context, write_buf, write_buf_len as _, None) };
            unsafe { tls_buffer_clear(client_context) }
        }
        let mut write_buf_len = 0;
        let write_buf = unsafe { tls_get_write_buffer(context, &raw mut write_buf_len) };
        println!("  server -> client {write_buf_len} bytes");
        if write_buf_len != 0 {
            unsafe { tls_consume_stream(client_context, write_buf, write_buf_len as _, None) };
            unsafe { tls_buffer_clear(context) }
        }
    }
    context
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let context = create_connection_context();
    let mut buf = vec![0; 4 << 10];
    c.bench_function("export", |b| {
        b.iter(|| unsafe { tls_export_context(context, buf.as_mut_ptr(), buf.len() as _, 1) })
    });

    let len = unsafe { tls_export_context(context, buf.as_mut_ptr(), buf.len() as _, 1) };
    println!("export size {len} bytes");

    c.bench_function("import", |b| {
        struct OwnedContext(*mut TLSContext);
        impl Drop for OwnedContext {
            fn drop(&mut self) {
                unsafe { tls_destroy_context(self.0) }
            }
        }
        b.iter(|| OwnedContext(unsafe { tls_import_context(buf.as_ptr(), len as _) }))
        // b.iter_with_large_drop(|| OwnedContext(unsafe { tls_import_context(buf.as_ptr(), len as _) }))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
