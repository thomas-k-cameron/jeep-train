use jeep_train_prelude::*;
use jeep_train_prelude as prelude;
use jeep_train_macro::{
    router,
    plugin,
    server
};

plugin! {
    const DEFAULT_RESPONSE;
    func set_404;
}

plugin! {
    const PRINT_REQ;
    func print_req;
}

router! {
    const ROUTER;
    scope "/" {
        get hello_world::index;
    }
    scope "/hello" {
        get hello_world::hello;
        scope "/:name" {
            get hello_world::greeting;
            scope "/:content_type" {
                get hello_world::content_type_greeting;
            }
        }
    }
}

server! {
    fn server_func;
    plugin PRINT_REQ;
    router ROUTER;
    plugin DEFAULT_RESPONSE;
}

fn print_req(conn: Conn) {
    println!();
    println!("Method: {:?}", conn.method());
    println!("Path: {}", conn.path());
    println!("Body: \n    {}", conn.body());
}

fn set_404(conn: Conn) {
    let mut resp = conn.mut_resp();

    // you can add a content type header on the response object on plugin and use that later!
    let body = match resp.headers.get("content-type") {
        Some(e) if e == "application/json" =>{
            r#"{"msg": "404 not found"}"#
        }
        _ => "404 not found"
    };

    resp.set_resp(404, body);

}

fn main() {
    println!("server will be running at localhost:3000");
    prelude::server::jeep_train("localhost:3000", server_func);
}

mod hello_world {
    use jeep_train_prelude::*;


    pub fn index(conn: Conn) {
        let mut resp = conn
            .mut_resp();

        resp.set_resp(200, "this is an index page");
        resp.set_headers("content-type".to_owned(), "text".to_owned());
    }

    pub fn hello(conn: Conn) {
        conn
            .mut_resp()
            .set_resp(200, "hello world");
    }

    pub fn greeting(conn: Conn) {
        let s= format!("Hi {}!", conn.path_info_match()[2]);
        conn.mut_resp()
            .set_resp(200, s);
    }

    pub fn content_type_greeting(conn: Conn) {
        let s= format!("Hi {}!", conn.path_info_match()[2]);

        let value = match conn.path_info_match()[3] {
            "json" => ("application/json", format!(r#"{{"msg": {:?}}}"#, s)),
            "xml" => ("application/xml", format!(r#"<MSG>{}</MSG>"#, s)),
            _ => ("text", s)
        };

        let mut resp = conn.mut_resp();
        resp.set_resp(200, value.1);
        resp.set_headers("content-type".to_owned(), value.0.to_owned());
    }

}