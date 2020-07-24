extern crate proc_macro;

mod template;
mod plugin;
mod router;
mod server;

use proc_macro::TokenStream;

/// keywords for plugin
/// ```
/// plugin!{
///     const PLUGIN;
///     func some_func;
///     func not_found;
/// }
/// ```
///
/// let's break down the example
/// - `const {ident};`
///
/// `const` keyword must come at the top level and it cannot be used more than once.
///
/// Ident given with the const keyword will become the name of the router as it expands into `const PLUGIN = // expression`
///
/// - `func {path expression};`
///
/// The proceeding path expression must point to a function
///
///
#[proc_macro]
pub fn plugin(ts: TokenStream) -> TokenStream {
    // moved the contents into ./plugin.rs
    plugin::main(ts)
}

/// Used to define function that acts as a server's logic.
/// Example
///
/// ```rust
/// server!{
///     fn server;
///     plugin CHECK_AUTH;
///     router USER::ROUTER;
///     router ARTICLE::ROUTER;
///     plugin DEFAULT_RESPONSE;
/// }
/// ```
///
/// Let's break down the example
///
/// - `fn server;`
///
/// `fn` keyword must come at the top level and it cannot be used twice or more.
///
/// Ident given with the const keyword become the name of the router as it expands into `fn server(conn: Conn) { /* expression */ }`
///
/// - `plugin {ident};`
///
/// `plugin` keyword is used to insert new plugin
///
/// - `router {ident};`
///
/// `router` keyword is used to insert new router
///
#[proc_macro]
pub fn server(input: TokenStream) -> TokenStream {
    server::main(input)
}

/// You use this to define router.
/// Macro will expand her input into match statement.
///
/// Example
/// ```rust
/// router! {
///     const ROUTER;
///
///     scope "/" {
///         get index;
///     }
///
///     scope "/hello" {
///         plugin LOGGER;
///         get hello;
///
///         scope "/:world" {
///             get greeting;
///         }
///     }
/// }
/// ```
///
/// let's break down the example
/// - `const ROUTER;`
///
/// `const` keyword must come at the top level and it cannot be used twice or more.
///
/// Ident given with the const keyword become the name of the router`
///
/// - `scope {lietral} {group}`
///
/// literal is the path, and the definition of the routes is defined at the group.
///
/// ## Keywords
/// ### plugin
///
/// There can be multiple plugins, but it won't be inherited to child scopes.
/// Plugin keyword must proceed with path expression that resolves to a function.
/// Resolved function will be invoked before the controller.
///
/// Example
/// ```rust
/// scope "/hello" {
///     plugin PLUGIN;
///     get hello;
/// }
/// ```
///
/// In this case, function `PLUGIN` will be invoked before `hello`.
///
/// - {http method in lower case} {ident};
///
/// Ident must be a function path. The function path will be invoked when the relevant request is received.
///
/// Example
/// ```rust
/// scope "/hello" {
///     get hello;
///     post new_greeting;
/// }
/// ```
/// Above will become
///
/// | method | path | function |
/// | --- | --- | --- |
/// | get | /hello | hello |
/// | post | /hello | new_greeting |
///
/// ### Keywords that defines multiple routes.
///
/// #### Resource
///
/// ```rust
/// scope "/resource" {
///     resource some_path_for_resource;
/// }
/// ```

/// | method | path | function |
/// | --- | --- | --- |
/// | get | /resource/ | some_path_for_resource::index |
/// | get | /resource/new | some_path_for_resource::new |
/// | post | /resource/ | some_path_for_resource::create |
/// | get | /resource/show | some_path_for_resource::show |
/// | get | /resource/:id/edit | some_path_for_resource::edit |
/// | put | /resource/:id | some_path_for_resource::update |
/// | patch | /resource/:id | some_path_for_resource::update |
/// | delete | /resource/ | some_path_for_resource::destroy |
///
/// Note that `/:id` is a parameterized segment
#[proc_macro]
pub fn router(input: TokenStream) -> TokenStream {
    router::main(input)
}
