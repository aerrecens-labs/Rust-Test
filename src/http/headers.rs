use tiny_http::Header;

use crate::config::Config;

pub fn json_headers(config: &Config) -> Vec<Header> {
    vec![
        Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
        Header::from_bytes(
            &b"Access-Control-Allow-Origin"[..],
            config.cors_origin.as_bytes(),
        )
        .unwrap(),
        Header::from_bytes(
            &b"Access-Control-Allow-Methods"[..],
            config.cors_methods.as_bytes(),
        )
        .unwrap(),
        Header::from_bytes(
            &b"Access-Control-Allow-Headers"[..],
            config.cors_headers.as_bytes(),
        )
        .unwrap(),
    ]
}
