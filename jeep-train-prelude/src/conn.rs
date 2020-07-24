use std::collections::HashMap;
use std::sync::{
    MutexGuard,
    Mutex,
    Arc
};

type Map = HashMap<String, String>;
pub type Conn = Arc<BearConnection>;
pub type Body = String;

#[derive(Default)]
pub struct BearConnection {
    // requested host
    pub method: Method,
    pub path: String,
    pub path_info: Vec<String>,
    pub req_query: Map,
    pub req_headers: Map,
    pub req_body: Body,
    // defining response data
    pub resp_config: Mutex<Resp>
}
impl BearConnection {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn headers(&self) -> &Map {
        &self.req_headers
    }

    pub fn body(&self) -> &Body {
        &self.req_body
    }

    pub fn path_info(&self) -> &Vec<String> {
        &self.path_info
    }

    pub fn path_info_match(&self) -> Vec<&str> {
        let mut vec = Vec::with_capacity(32);
        for i in self.path_info().iter() {
            vec.push(i.as_str())
        }
        vec
    }

    // responses
    pub fn mut_resp(&self) -> MutexGuard<Resp> {
        self
            .resp_config
            .lock()
            .unwrap()
    }

    pub fn halt(&self) -> bool {
        match self.resp_config.lock() {
            Ok(e) => {
                e.is_reply_set
            },
            Err(_) => false
        }
    }
}

// default was implemented manually
pub struct Resp {
    pub status: u16,
    pub headers: Map,
    pub body: String,
    pub is_reply_set: bool
}

impl Resp {
    pub fn set_resp(&mut self, status: u16, body: impl ToString) {
        self.status = status;
        self.body = body.to_string();
        self.is_reply_set = true;
    }
    pub fn set_headers(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

impl Default for Resp {
    fn default() -> Self {
        Self {
            status: 404,
            headers: Map::with_capacity(16),
            body: String::with_capacity(2 * 1024),
            is_reply_set: false
        }
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    HEAD,
    TRACE,
    PATCH,
    DELETE,
    OPTIONS,
    CONNECT,
    Unknown(Box<String>)
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}
