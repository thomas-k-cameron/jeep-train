use jeep_train_prelude::*;
use jeep_train_macro::*;
use jeep_train_prelude::server::jeep_train;

fn set_text_header(conn: Conn) {
    conn
        .mut_resp()
        .set_headers("content-type".to_owned(), "text".to_owned());
}

fn default(conn: Conn) {
    conn.mut_resp()
        .set_resp(404, "not found");
}

fn lucky_seven(conn: Conn) {
    if conn.path().len() == 7 {
        conn.mut_resp()
            .set_resp(200, "lucky seven!")
    }
}

fn reject_swear_words(conn: Conn) {
    if conn.path().contains("fuck") {
        conn.mut_resp()
            .set_resp(400, "don't swear")
    }
}

plugin! {
    const DEFAULT_RESPONSE;
    func lucky_seven;
    func default;
}

router! {
    const RESOURCE_ROUTER;
    scope "/resource" {
        plugin set_text_header;
        resource resource_module;
    }
}

server! {
    fn resource_server;
    plugin reject_swear_words;
    router RESOURCE_ROUTER;
    plugin DEFAULT_RESPONSE;
}

fn main() {
    println!("server will be running at localhost:3000");
    jeep_train("localhost:3000", resource_server);
}

pub mod resource_module {
    use super::*;

    macro_rules! set {
        ($conn:tt, $lit:literal) => {
            $conn
                .mut_resp()
                .set_resp(200, $lit);
        };
    }

    pub fn index(conn: Conn) {
        set!(conn, "this is an index page");
    }

    pub fn new(conn: Conn) {
        set!(conn, "anything new yet?");
    }

    pub fn create(conn: Conn) {
        set!(conn, "you can't create anything. sorry!");
    }

    pub fn show(conn: Conn) {
        set!(conn, "I wish that I could have shown you a cat video!");
    }

    pub fn update(conn: Conn) {
        set!(conn, "nothing to update!");
    }

    pub fn destroy(conn: Conn) {
        set!(conn, "you cannot delete it!");
    }

    pub fn edit(conn: Conn) {
        set!(conn, "you cannot edit anything");
    }
}


