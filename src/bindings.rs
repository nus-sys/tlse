/* automatically generated by rust-bindgen 0.70.1 */

pub const SSL_V30: u32 = 768;
pub const TLS_V10: u32 = 769;
pub const TLS_V11: u32 = 770;
pub const TLS_V12: u32 = 771;
pub const TLS_V13: u32 = 772;
pub const DTLS_V10: u32 = 65279;
pub const DTLS_V12: u32 = 65277;
pub const DTLS_V13: u32 = 65276;
pub const TLS_NEED_MORE_DATA: u32 = 0;
pub const TLS_GENERIC_ERROR: i32 = -1;
pub const TLS_BROKEN_PACKET: i32 = -2;
pub const TLS_NOT_UNDERSTOOD: i32 = -3;
pub const TLS_NOT_SAFE: i32 = -4;
pub const TLS_NO_COMMON_CIPHER: i32 = -5;
pub const TLS_UNEXPECTED_MESSAGE: i32 = -6;
pub const TLS_CLOSE_CONNECTION: i32 = -7;
pub const TLS_COMPRESSION_NOT_SUPPORTED: i32 = -8;
pub const TLS_NO_MEMORY: i32 = -9;
pub const TLS_NOT_VERIFIED: i32 = -10;
pub const TLS_INTEGRITY_FAILED: i32 = -11;
pub const TLS_ERROR_ALERT: i32 = -12;
pub const TLS_BROKEN_CONNECTION: i32 = -13;
pub const TLS_BAD_CERTIFICATE: i32 = -14;
pub const TLS_UNSUPPORTED_CERTIFICATE: i32 = -15;
pub const TLS_NO_RENEGOTIATION: i32 = -16;
pub const TLS_FEATURE_NOT_SUPPORTED: i32 = -17;
pub const TLS_DECRYPTION_FAILED: i32 = -20;
pub const TLS_AES_128_GCM_SHA256: u32 = 4865;
pub const TLS_AES_256_GCM_SHA384: u32 = 4866;
pub const TLS_CHACHA20_POLY1305_SHA256: u32 = 4867;
pub const TLS_AES_128_CCM_SHA256: u32 = 4868;
pub const TLS_AES_128_CCM_8_SHA256: u32 = 4869;
pub const TLS_RSA_WITH_AES_128_CBC_SHA: u32 = 47;
pub const TLS_RSA_WITH_AES_256_CBC_SHA: u32 = 53;
pub const TLS_RSA_WITH_AES_128_CBC_SHA256: u32 = 60;
pub const TLS_RSA_WITH_AES_256_CBC_SHA256: u32 = 61;
pub const TLS_RSA_WITH_AES_128_GCM_SHA256: u32 = 156;
pub const TLS_RSA_WITH_AES_256_GCM_SHA384: u32 = 157;
pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA: u32 = 51;
pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA: u32 = 57;
pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA256: u32 = 103;
pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA256: u32 = 107;
pub const TLS_DHE_RSA_WITH_AES_128_GCM_SHA256: u32 = 158;
pub const TLS_DHE_RSA_WITH_AES_256_GCM_SHA384: u32 = 159;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: u32 = 49171;
pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA: u32 = 49172;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256: u32 = 49191;
pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: u32 = 49199;
pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: u32 = 49200;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: u32 = 49161;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: u32 = 49162;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256: u32 = 49187;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384: u32 = 49188;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: u32 = 49195;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: u32 = 49196;
pub const TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: u32 = 52392;
pub const TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: u32 = 52393;
pub const TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256: u32 = 52394;
pub const TLS_FALLBACK_SCSV: u32 = 22016;
pub const TLS_UNSUPPORTED_ALGORITHM: u32 = 0;
pub const TLS_RSA_SIGN_RSA: u32 = 1;
pub const TLS_RSA_SIGN_MD5: u32 = 4;
pub const TLS_RSA_SIGN_SHA1: u32 = 5;
pub const TLS_RSA_SIGN_SHA256: u32 = 11;
pub const TLS_RSA_SIGN_SHA384: u32 = 12;
pub const TLS_RSA_SIGN_SHA512: u32 = 13;
pub const TLS_ECDSA_SIGN_SHA256: u32 = 14;
pub const TLS_EC_PUBLIC_KEY: u32 = 17;
pub const TLS_EC_prime192v1: u32 = 18;
pub const TLS_EC_prime192v2: u32 = 19;
pub const TLS_EC_prime192v3: u32 = 20;
pub const TLS_EC_prime239v1: u32 = 21;
pub const TLS_EC_prime239v2: u32 = 22;
pub const TLS_EC_prime239v3: u32 = 23;
pub const TLS_EC_prime256v1: u32 = 24;
pub const TLS_EC_secp224r1: u32 = 21;
pub const TLS_EC_secp256r1: u32 = 23;
pub const TLS_EC_secp384r1: u32 = 24;
pub const TLS_EC_secp521r1: u32 = 25;
pub const TLS_ALERT_WARNING: u32 = 1;
pub const TLS_ALERT_CRITICAL: u32 = 2;
pub const SRTP_AES128_CM_HMAC_SHA1_80: u32 = 1;
pub const SRTP_AES128_CM_HMAC_SHA1_32: u32 = 2;
pub const SRTP_NULL_HMAC_SHA1_80: u32 = 5;
pub const SRTP_NULL_HMAC_SHA1_32: u32 = 6;
pub const SRTP_AEAD_AES_128_GCM: u32 = 7;
pub const SRTP_AEAD_AES_256_GCM: u32 = 8;
pub const SRTP_NULL: u32 = 0;
pub const SRTP_AES_CM: u32 = 1;
pub const SRTP_AUTH_NULL: u32 = 0;
pub const SRTP_AUTH_HMAC_SHA1: u32 = 1;
pub const SSL_SERVER_RSA_CERT: u32 = 1;
pub const SSL_SERVER_RSA_KEY: u32 = 2;
pub const SSL_FILETYPE_PEM: u32 = 1;
pub const SSL_VERIFY_NONE: u32 = 0;
pub const SSL_VERIFY_PEER: u32 = 1;
pub const SSL_VERIFY_FAIL_IF_NO_PEER_CERT: u32 = 2;
pub const SSL_VERIFY_CLIENT_ONCE: u32 = 3;
pub const TLSAlertDescription_close_notify: TLSAlertDescription = 0;
pub const TLSAlertDescription_unexpected_message: TLSAlertDescription = 10;
pub const TLSAlertDescription_bad_record_mac: TLSAlertDescription = 20;
pub const TLSAlertDescription_decryption_failed_RESERVED: TLSAlertDescription = 21;
pub const TLSAlertDescription_record_overflow: TLSAlertDescription = 22;
pub const TLSAlertDescription_decompression_failure: TLSAlertDescription = 30;
pub const TLSAlertDescription_handshake_failure: TLSAlertDescription = 40;
pub const TLSAlertDescription_no_certificate_RESERVED: TLSAlertDescription = 41;
pub const TLSAlertDescription_bad_certificate: TLSAlertDescription = 42;
pub const TLSAlertDescription_unsupported_certificate: TLSAlertDescription = 43;
pub const TLSAlertDescription_certificate_revoked: TLSAlertDescription = 44;
pub const TLSAlertDescription_certificate_expired: TLSAlertDescription = 45;
pub const TLSAlertDescription_certificate_unknown: TLSAlertDescription = 46;
pub const TLSAlertDescription_illegal_parameter: TLSAlertDescription = 47;
pub const TLSAlertDescription_unknown_ca: TLSAlertDescription = 48;
pub const TLSAlertDescription_access_denied: TLSAlertDescription = 49;
pub const TLSAlertDescription_decode_error: TLSAlertDescription = 50;
pub const TLSAlertDescription_decrypt_error: TLSAlertDescription = 51;
pub const TLSAlertDescription_export_restriction_RESERVED: TLSAlertDescription = 60;
pub const TLSAlertDescription_protocol_version: TLSAlertDescription = 70;
pub const TLSAlertDescription_insufficient_security: TLSAlertDescription = 71;
pub const TLSAlertDescription_internal_error: TLSAlertDescription = 80;
pub const TLSAlertDescription_inappropriate_fallback: TLSAlertDescription = 86;
pub const TLSAlertDescription_user_canceled: TLSAlertDescription = 90;
pub const TLSAlertDescription_no_renegotiation: TLSAlertDescription = 100;
pub const TLSAlertDescription_unsupported_extension: TLSAlertDescription = 110;
pub const TLSAlertDescription_no_error: TLSAlertDescription = 255;
pub type TLSAlertDescription = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TLSPacket {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TLSCertificate {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TLSContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ECCCurveParameters {
    _unused: [u8; 0],
}
pub type TLS = TLSContext;
pub type Certificate = TLSCertificate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TLSRTCPeerConnection {
    _unused: [u8; 0],
}
pub type tls_validation_function = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut TLSContext,
        certificate_chain: *mut *mut TLSCertificate,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn tls_init();
}
extern "C" {
    pub fn tls_pem_decode(
        data_in: *const ::std::os::raw::c_uchar,
        input_length: ::std::os::raw::c_uint,
        cert_index: ::std::os::raw::c_int,
        output_len: *mut ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn tls_create_certificate() -> *mut TLSCertificate;
}
extern "C" {
    pub fn tls_certificate_valid_subject(
        cert: *mut TLSCertificate,
        subject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_valid_subject_name(
        cert_subject: *const ::std::os::raw::c_uchar,
        subject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_is_valid(cert: *mut TLSCertificate) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_set_copy(
        member: *mut *mut ::std::os::raw::c_uchar,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_copy_date(
        member: *mut *mut ::std::os::raw::c_uchar,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_key(
        cert: *mut TLSCertificate,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_priv(
        cert: *mut TLSCertificate,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_sign_key(
        cert: *mut TLSCertificate,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_to_string(
        cert: *mut TLSCertificate,
        buffer: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_certificate_set_exponent(
        cert: *mut TLSCertificate,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_serial(
        cert: *mut TLSCertificate,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_certificate_set_algorithm(
        context: *mut TLSContext,
        algorithm: *mut ::std::os::raw::c_uint,
        val: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_destroy_certificate(cert: *mut TLSCertificate);
}
extern "C" {
    pub fn tls_create_packet(
        context: *mut TLSContext,
        type_: ::std::os::raw::c_uchar,
        version: ::std::os::raw::c_ushort,
        payload_size_hint: ::std::os::raw::c_int,
    ) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_destroy_packet(packet: *mut TLSPacket);
}
extern "C" {
    pub fn tls_packet_update(packet: *mut TLSPacket);
}
extern "C" {
    pub fn tls_packet_append(
        packet: *mut TLSPacket,
        buf: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_packet_uint8(
        packet: *mut TLSPacket,
        i: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_packet_uint16(
        packet: *mut TLSPacket,
        i: ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_packet_uint32(
        packet: *mut TLSPacket,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_packet_uint24(
        packet: *mut TLSPacket,
        i: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_random(
        key: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_get_write_buffer(
        context: *mut TLSContext,
        outlen: *mut ::std::os::raw::c_uint,
    ) -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn tls_buffer_clear(context: *mut TLSContext);
}
extern "C" {
    pub fn tls_established(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_read_clear(context: *mut TLSContext);
}
extern "C" {
    pub fn tls_read(
        context: *mut TLSContext,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_create_context(
        is_server: ::std::os::raw::c_uchar,
        version: ::std::os::raw::c_ushort,
    ) -> *mut TLSContext;
}
extern "C" {
    pub fn tls_set_curve(
        context: *mut TLSContext,
        curve: *const ECCCurveParameters,
    ) -> *const ECCCurveParameters;
}
extern "C" {
    pub fn tls_accept(context: *mut TLSContext) -> *mut TLSContext;
}
extern "C" {
    pub fn tls_set_default_dhe_pg(
        context: *mut TLSContext,
        p_hex_str: *const ::std::os::raw::c_char,
        g_hex_str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_destroy_context(context: *mut TLSContext);
}
extern "C" {
    pub fn tls_cipher_supported(
        context: *mut TLSContext,
        cipher: ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_cipher_is_fs(
        context: *mut TLSContext,
        cipher: ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_choose_cipher(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        scsv_set: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_cipher_is_ephemeral(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_cipher_name(context: *mut TLSContext) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_is_ecdsa(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_build_client_key_exchange(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_server_key_exchange(
        context: *mut TLSContext,
        method: ::std::os::raw::c_int,
    ) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_hello(
        context: *mut TLSContext,
        tls13_downgrade: ::std::os::raw::c_int,
    ) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_certificate_request(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_verify_request(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_parse_hello(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        write_packets: *mut ::std::os::raw::c_uint,
        dtls_verified: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_certificate(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        is_client: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_server_key_exchange(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_client_key_exchange(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_server_hello_done(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_finished(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        write_packets: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_verify(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_payload(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        certificate_verify: tls_validation_function,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_parse_message(
        context: *mut TLSContext,
        buf: *mut ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        certificate_verify: tls_validation_function,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_verify_signature(
        cert: *mut TLSCertificate,
        parent: *mut TLSCertificate,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_chain_is_valid(
        certificates: *mut *mut TLSCertificate,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_certificate_chain_is_valid_root(
        context: *mut TLSContext,
        certificates: *mut *mut TLSCertificate,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_load_certificates(
        context: *mut TLSContext,
        pem_buffer: *const ::std::os::raw::c_uchar,
        pem_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_load_private_key(
        context: *mut TLSContext,
        pem_buffer: *const ::std::os::raw::c_uchar,
        pem_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_build_certificate(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_finished(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_change_cipher_spec(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_done(context: *mut TLSContext) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_build_message(
        context: *mut TLSContext,
        data: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uint,
    ) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_client_connect(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_write(
        context: *mut TLSContext,
        data: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_build_alert(
        context: *mut TLSContext,
        critical: ::std::os::raw::c_char,
        code: ::std::os::raw::c_uchar,
    ) -> *mut TLSPacket;
}
extern "C" {
    pub fn tls_consume_stream(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        certificate_verify: tls_validation_function,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_close_notify(context: *mut TLSContext);
}
extern "C" {
    pub fn tls_alert(
        context: *mut TLSContext,
        critical: ::std::os::raw::c_uchar,
        code: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn tls_pending(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_make_exportable(context: *mut TLSContext, exportable_flag: ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn tls_export_context(
        context: *mut TLSContext,
        buffer: *mut ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_uint,
        small_version: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_import_context(
        buffer: *const ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_uint,
    ) -> *mut TLSContext;
}
extern "C" {
    pub fn tls_is_broken(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_request_client_certificate(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_client_verified(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_sni(context: *mut TLSContext) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_sni_set(
        context: *mut TLSContext,
        sni: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_sni_nset(
        context: *mut TLSContext,
        sni: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_srtp_set(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_srtp_key(
        context: *mut TLSContext,
        buffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_stun_parse(
        msg: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
        pwd: *mut ::std::os::raw::c_char,
        pwd_len: ::std::os::raw::c_int,
        is_ipv6: ::std::os::raw::c_uchar,
        addr: *mut ::std::os::raw::c_uchar,
        port: ::std::os::raw::c_uint,
        response_buffer: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_stun_build(
        transaction_id: *mut ::std::os::raw::c_uchar,
        username: *mut ::std::os::raw::c_char,
        username_len: ::std::os::raw::c_int,
        pwd: *mut ::std::os::raw::c_char,
        pwd_len: ::std::os::raw::c_int,
        msg: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_is_stun(
        msg: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type tls_peerconnection_write_function = ::std::option::Option<
    unsafe extern "C" fn(
        channel: *mut TLSRTCPeerConnection,
        msg: *const ::std::os::raw::c_uchar,
        msg_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn tls_peerconnection_context(
        active: ::std::os::raw::c_uchar,
        certificate_verify: tls_validation_function,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut TLSRTCPeerConnection;
}
extern "C" {
    pub fn tls_peerconnection_duplicate(
        channel: *mut TLSRTCPeerConnection,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut TLSRTCPeerConnection;
}
extern "C" {
    pub fn tls_peerconnection_dtls_context(channel: *mut TLSRTCPeerConnection) -> *mut TLSContext;
}
extern "C" {
    pub fn tls_peerconnection_remote_credentials(
        channel: *mut TLSRTCPeerConnection,
        remote_username: *mut ::std::os::raw::c_char,
        remote_username_len: ::std::os::raw::c_int,
        remote_pwd: *mut ::std::os::raw::c_char,
        remote_pwd_len: ::std::os::raw::c_int,
        remote_fingerprint: *mut ::std::os::raw::c_char,
        remote_fingerprint_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_local_pwd(
        channel: *mut TLSRTCPeerConnection,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_peerconnection_local_username(
        channel: *mut TLSRTCPeerConnection,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_peerconnection_userdata(
        channel: *mut TLSRTCPeerConnection,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn tls_peerconnection_load_keys(
        channel: *mut TLSRTCPeerConnection,
        pem_pub_key: *const ::std::os::raw::c_uchar,
        pem_pub_key_size: ::std::os::raw::c_int,
        pem_priv_key: *const ::std::os::raw::c_uchar,
        pem_priv_key_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_connect(
        channel: *mut TLSRTCPeerConnection,
        write_function: tls_peerconnection_write_function,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_iterate(
        channel: *mut TLSRTCPeerConnection,
        buf: *mut ::std::os::raw::c_uchar,
        buf_len: ::std::os::raw::c_int,
        addr: *mut ::std::os::raw::c_uchar,
        port: ::std::os::raw::c_int,
        is_ipv6: ::std::os::raw::c_uchar,
        write_function: tls_peerconnection_write_function,
        validate_addr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_get_write_msg(
        channel: *mut TLSRTCPeerConnection,
        buf: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_get_read_msg(
        channel: *mut TLSRTCPeerConnection,
        buf: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_peerconnection_status(channel: *mut TLSRTCPeerConnection) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_destroy_peerconnection(channel: *mut TLSRTCPeerConnection);
}
extern "C" {
    pub fn tls_cert_fingerprint(
        pem_data: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_char,
        buf_len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_load_root_certificates(
        context: *mut TLSContext,
        pem_buffer: *const ::std::os::raw::c_uchar,
        pem_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_default_verify(
        context: *mut TLSContext,
        certificate_chain: *mut *mut TLSCertificate,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_print_certificate(fname: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn tls_add_alpn(
        context: *mut TLSContext,
        alpn: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_alpn_contains(
        context: *mut TLSContext,
        alpn: *const ::std::os::raw::c_char,
        alpn_size: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_alpn(context: *mut TLSContext) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tls_clear_certificates(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_make_ktls(
        context: *mut TLSContext,
        socket: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tls_unmake_ktls(
        context: *mut TLSContext,
        socket: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dtls_reset_cookie_secret();
}
extern "C" {
    pub fn tls_remote_error(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
pub type SSL_CTX = TLSContext;
pub type SSL = TLSContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SSLUserData {
    pub fd: ::std::os::raw::c_int,
    pub certificate_verify: tls_validation_function,
    pub recv: *mut ::std::os::raw::c_void,
    pub send: *mut ::std::os::raw::c_void,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SSLUserData"][::std::mem::size_of::<SSLUserData>() - 40usize];
    ["Alignment of SSLUserData"][::std::mem::align_of::<SSLUserData>() - 8usize];
    ["Offset of field: SSLUserData::fd"][::std::mem::offset_of!(SSLUserData, fd) - 0usize];
    ["Offset of field: SSLUserData::certificate_verify"]
        [::std::mem::offset_of!(SSLUserData, certificate_verify) - 8usize];
    ["Offset of field: SSLUserData::recv"][::std::mem::offset_of!(SSLUserData, recv) - 16usize];
    ["Offset of field: SSLUserData::send"][::std::mem::offset_of!(SSLUserData, send) - 24usize];
    ["Offset of field: SSLUserData::user_data"]
        [::std::mem::offset_of!(SSLUserData, user_data) - 32usize];
};
extern "C" {
    pub fn SSL_library_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_load_error_strings();
}
extern "C" {
    pub fn OpenSSL_add_all_algorithms();
}
extern "C" {
    pub fn OpenSSL_add_all_ciphers();
}
extern "C" {
    pub fn OpenSSL_add_all_digests();
}
extern "C" {
    pub fn EVP_cleanup();
}
extern "C" {
    pub fn SSLv3_server_method() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSLv3_client_method() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_new(context: *mut TLSContext) -> *mut TLSContext;
}
extern "C" {
    pub fn SSL_CTX_use_certificate_file(
        context: *mut TLSContext,
        filename: *const ::std::os::raw::c_char,
        dummy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_CTX_use_PrivateKey_file(
        context: *mut TLSContext,
        filename: *const ::std::os::raw::c_char,
        dummy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_CTX_check_private_key(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_CTX_new(method: ::std::os::raw::c_int) -> *mut TLSContext;
}
extern "C" {
    pub fn SSL_free(context: *mut TLSContext);
}
extern "C" {
    pub fn SSL_CTX_free(context: *mut TLSContext);
}
extern "C" {
    pub fn SSL_get_error(
        context: *mut TLSContext,
        ret: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_set_fd(
        context: *mut TLSContext,
        socket: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_set_userdata(
        context: *mut TLSContext,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn SSL_userdata(context: *mut TLSContext) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn SSL_CTX_root_ca(
        context: *mut TLSContext,
        pem_filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_CTX_set_verify(
        context: *mut TLSContext,
        mode: ::std::os::raw::c_int,
        verify_callback: tls_validation_function,
    );
}
extern "C" {
    pub fn SSL_accept(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_connect(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_shutdown(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_write(
        context: *mut TLSContext,
        buf: *const ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_read(
        context: *mut TLSContext,
        buf: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_pending(context: *mut TLSContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SSL_set_io(
        context: *mut TLSContext,
        recv: *mut ::std::os::raw::c_void,
        send: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
