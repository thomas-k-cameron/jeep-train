# About
## Overview
Jeep train is an experimental high level web framework

The project is purely experimental and it is not meant to be used in a production environment.  

The motivation behind the development is to 
- see if procedural macro can help define a runtime efficient router productively
- explore ways to minimize boilerplate/learning cost

## Feature
- Define your api routes on router that works with rails-like syntax. API Syntax is straight forward and inputs are converted into match statement
- `Conn` object will be passed onto every functions invoked in the process of handling client request and it will act as a primary interface for interacting with request/response data

## Routing 
It looks like this.

```rust
router! {
    const NAME_OF_THE_ROUTER;
    scope "/api" {
        scope "/v1" {
            get api::v1::get;
            post api::v1::create;
            delete api::v1::destroy;
        }
    }
    scope "/resource" {
        resource index::resource;
    }
}
```

# Example
This code can be found on `examples/resources`.

```rust
use jeep_train_prelude::*;
use jeep_train_macro::{ router, server, plugin };
use jeep_train_prelude::server::jeep_train;

fn set_text_header(conn: Conn) {
    conn
        // obtain a mutex on response object for the user
        .mut_resp() 
        // insert a header data 
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
            .set_resp(200, "don't swear")
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

/// # Routes that it creates
/// 
/// | method | path | function |
/// | --- | --- | --- |
/// | get | /resource/ | resource_module::index |
/// | get | /resource/new | resource_module::new |
/// | post | /resource/ | resource_module::create |
/// | get | /resource/show | resource_module::show |
/// | get | /resource/:id/edit | resource_module::edit |
/// | put | /resource/:id | resource_module::update |
/// | patch | /resource/:id | resource_module::update |
/// | delete | /resource/ | resource_module::destroy |
///
/// Note that `/:id` is a parameterized segment
/// 
fn main() {
    jeep_train("localhost:3000", resource_server);
}

pub mod resource_module {
   /* ...functions  */
}
```

## Benchmark
Jeep train doesn't use regular expression.   
However, I think it's not common to create a parameterized path with complicated logic.
```
running 2 tests
test router_actix      ... bench:     836,812 ns/iter (+/- 22,033)
test router_jeep_train ... bench:       1,491 ns/iter (+/- 51)
```
Code and details can be found in benchmark-result

