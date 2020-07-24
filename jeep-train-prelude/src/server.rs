use std::sync::Arc;
use std::collections::{
    HashMap,
};

use self::super::*;

use tiny_http;
use tiny_http::{
    Header,
    Response,
    StatusCode
};

type Logic = fn (conn: Conn) -> ();

// this function is separated from jeep_train because it used to be exported
fn invoke(mut req: tiny_http::Request, logic: Logic) {
    // construct bear connection
    let bear = {
        use std::sync::Mutex;

        let method = {
            use tiny_http::Method as H;
            use self::Method::*;
            match req.method() {
                H::Get => GET,
                H::Post => POST,
                H::Put => PUT,
                H::Patch => PATCH,
                H::Delete => DELETE,
                H::Head => HEAD,
                H::Trace => TRACE,
                H::Options => OPTIONS,
                H::Connect => CONNECT,
                H::NonStandard(s) => Unknown(Box::new(s.to_string()))
            }
        };

        let mut req_body = String::with_capacity(3 * 1024);
        let _ = req.as_reader().read_to_string(&mut req_body).unwrap();

        let req_headers = {
            let mut tree = HashMap::with_capacity(32);
            for i in req.headers() {
                tree.insert(
                    i.field.as_str().to_string(),
                    i.value.as_str().to_string()
                );
            }
            tree
        };

        let (path, req_query) = {
            let mut counter = 0;
            for b in req.url().as_bytes() {
                if b == &b"?"[0] {
                    break;
                }
                counter += 1;
            }
            let path = req.url()[0..counter].to_string();

            let mut req_query = HashMap::with_capacity(32);
            if req.url().len() > counter {
                if let Ok(data) = serde_urlencoded::from_str(&req.url()[counter..]) {
                    let mut vec: Vec<(String, String)> = data;
                    vec.drain(..).for_each(|(k, v)| {
                        req_query.insert(k, v);
                    });
                }
            };

            (path, req_query)
        };

        let path_info = path_info::runtime_new(&path);

        BearConnection {
            method,
            path,
            path_info,
            req_query,
            req_body,
            req_headers,
            resp_config: Mutex::new(Resp::default())
        }
    };

    let conn = Arc::new(bear);

    logic(conn.clone());

    // lock happens here because it won't live long enough unless so
    let resp = conn.mut_resp();

    // create a　response payload out of
    let payload = {
        let status = StatusCode(resp.status);
        let headers = {
            let mut headers = Vec::with_capacity(32);
            for i in resp.headers.iter() {
                let head =
                    Header::from_bytes(i.0.as_bytes(), i.1.as_bytes()).unwrap();
                headers.push(head)
            }
            headers
        };

        Response::new(
            status,
            headers,
            resp.body.as_bytes(),
            Some(resp.body.len()),
            None
        )
    };

    req.respond(payload).unwrap();

}

pub fn jeep_train(host_addr: &str, logic: Logic) {
    let server = tiny_http::Server::http(host_addr).unwrap();

    loop {
        let req = match server.recv() {
            Ok(req) => req,
            Err(e) => {
                println!("{:?}", e);
                continue
            }
        };

        invoke(req, logic);
    }
}