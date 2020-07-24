use proc_macro::TokenStream;
use std::str::FromStr;

pub const SCOPE_METHODS: [&str; 9] = [
    "get",
    "put",
    "post",
    "head",
    "trace",
    "patch",
    "delete",
    "options",
    "connect"
];

pub const PRIMARY_METHODS: [&str; 5] = [
    "get",
    "put",
    "post",
    "patch",
    "delete"
];


pub fn conn_halt() -> TokenStream {
    let code = r#"
                    if conn.halt() {
                        return;
                    };
    "#;
    TokenStream::from_str(code).unwrap()
}

pub fn conn_call() -> TokenStream {
    TokenStream::from_str("(conn.clone());").unwrap()
}

pub fn return_false() -> TokenStream {
    TokenStream::from_str("return false;").unwrap()
}