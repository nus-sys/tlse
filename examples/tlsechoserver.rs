// CERT_FILE=examples/cert/server.crt KEY_FILE=examples/cert/server.key cargo run -r --example tlsechoserver
use std::{
    env::var,
    fs::read,
    io::{Read, Write},
    net::TcpListener,
    slice,
};

use tlse::{
    tls_accept, tls_buffer_clear, tls_consume_stream, tls_create_context, tls_destroy_context,
    tls_established, tls_get_write_buffer, tls_load_certificates, tls_load_private_key, tls_read,
    tls_write, TLS_V12,
};

fn main() {
    let server_context = unsafe { tls_create_context(1, TLS_V12 as _) };
    let cert_file = var("CERT_FILE").unwrap_or("./examples/cert/server.crt".into());
    let key_file = var("KEY_FILE").unwrap_or("./examples/cert/server.key".into());
    let cert = read(cert_file).expect("can read certificate file");
    unsafe { tls_load_certificates(server_context, cert.as_ptr(), cert.len() as _) };
    let key = read(key_file).expect("can read key file");
    unsafe { tls_load_private_key(server_context, key.as_ptr(), key.len() as _) };

    let listener = TcpListener::bind("0.0.0.0:8000").expect("can listen");
    loop {
        let (mut conn, _) = listener.accept().expect("can accept");
        let context = unsafe { tls_accept(server_context) };

        let mut buf = vec![0; 4 << 10];
        loop {
            let len = conn.read(&mut buf).expect("can read");
            if len == 0 {
                break;
            }
            unsafe { tls_consume_stream(context, buf.as_mut_ptr(), len as _, None) };

            let mut read_buf = vec![0; 4 << 10];
            if unsafe { tls_established(context) } != 0 {
                let len = unsafe { tls_read(context, read_buf.as_mut_ptr(), read_buf.len() as _) };
                unsafe { tls_write(context, read_buf.as_ptr(), len as _) };
            }

            let mut write_buf_len = 0u32;
            let write_buf_ptr = unsafe { tls_get_write_buffer(context, &mut write_buf_len as _) };
            if write_buf_len != 0 {
                conn.write_all(unsafe { slice::from_raw_parts(write_buf_ptr, write_buf_len as _) })
                    .expect("can write");
                unsafe { tls_buffer_clear(context) }
            }
        }
        unsafe { tls_destroy_context(context) }
    }
    // TODO destroy server context
}
