// TODO: FIX THE ALGORITHM
// I decided to separated functions into different bits as the code has gone bigger
use jeep_train_prelude::*;
use proc_macro::{TokenStream, TokenTree};

use crate::template;

use path_info::{
    // PathInfo,
    PathElm
};

use std::collections::{
    BTreeMap,
    BTreeSet
};

pub fn main(input: TokenStream) -> TokenStream {
    let expanded = {
        let mut counter = 1;
        // function for parsing routes specified with nested_routes;
        let func = |parent: &Scope| {
            let mut got = vec![];
            for (x, ts) in parent.nested_routes.iter() {
                let mut vec_path = parent.path.clone();
                vec_path.push(x.clone());
                let scope = Scope::parse(vec_path, ts.clone());
                got.push(scope);
            }
            got
        };

        // parse root scpoe
        let mut scopes = BTreeMap::new();
        let mut checked = BTreeSet::new();

        // root scopes

        let root_scope = Scope::parse(vec!["".to_owned()], input);
        scopes.insert(counter, root_scope);

        let mut temp = vec![];
        loop {
            for x in scopes.iter_mut() {
                let (id, s) = x;

                if checked.contains(id) {
                    continue
                }

                func(s)
                    .drain(..)
                    .into_iter()
                    .for_each(|f| temp.push((*id, f)));

                checked.insert(*id);
            }

            temp.drain(..).for_each(|(id, mut s)| {
                counter += 1;
                s.parent = id;
                scopes.insert(counter, s);
            });

            if checked.len() == scopes.len() {
                break;
            }
        }

        scopes
    };

    let mut dispatches = {
        let mut dispatches = vec![];
        let mut attachments = vec![];
        let mut d_id = 0;
        for (_id, s) in expanded.iter() {

            for (k, v) in s.declr_found.iter() {
                d_id += 1;

                let reg_type = if template::SCOPE_METHODS.contains(&k.as_str()) {
                    RegType::MethodName
                } else if k == "methods" {
                    RegType::Methods
                } else if k == "resource" {
                    RegType::Resource
                } else {
                    d_id -= 1;
                    continue
                };

                // concat path
                let path = format!{ "{}", &s.path.join("") };
                // creat path info
                let path_info = path_info::compile_time_new(&path);
                let d = Dispatch {
                    id: d_id,
                    path,
                    func: v.to_owned(),
                    method: k.to_owned(),
                    reg_type,
                    path_info,
                    attachments: attachments.clone(),
                    pairs: s.declr_found.clone()
                };
                dispatches.push(d);

                attachments.clear();
            }
        }

        let mut d_vec = vec![];
        let mut counter = IdCounter(d_id);
        for i in dispatches {
            if i.reg_type == RegType::MethodName {
                d_vec.push(i);
            } else {
                d_vec.append(&mut i.expand(&mut counter));
            }
        }
        d_vec
    };

    let (static_finalized, dynamic_finalized) = {
        // create foreign keys
        let mut methods = RelIndex::default();
        let mut static_length = RelIndex::default();
        let mut segment_count = RelIndex::default();
        let mut tree = BTreeMap::new();

        for d in dispatches.drain(..) {
            methods.insert(&d.method, d.id);

            if !d.path_info.iter().any(|(p, _s)| &PathElm::Dynamic == p) {
                static_length.insert(d.path.len(), d.id);
            } else {
                segment_count.insert(d.path_info.len(), d.id);
            };

            tree.insert(d.id, d);
        }

        // I'm not using vec![] to avoid warning
        let mut keys_got: Vec<usize>;
        // create path => func pair
        let mut method_match = MakeMatch::default();
        let mut len_match = MakeMatch::default();
        let mut path_match = MakeMatch::default();

        // for dynamic
        let mut method_match_for_seg = MakeMatch::default();
        let mut segment_match = MakeMatch::default();
        let mut segment_count_match = MakeMatch::default();

        let assign_plugin = |d: &Dispatch| {
            d.pairs.iter()
                .filter(|(k, _v)| k == "plugin")
                .map(|p| {
                    format!{r#"
                        {func}(conn.clone());
                        if conn.halt() {{
                            return true
                        }}
                    "#, func = p.1
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        };

        for (method, mset) in methods.set.iter() {

            for (seg_count, segset) in segment_count.set.iter() {
                keys_got = segset
                    .iter()
                    .map(|i| *i)
                    .filter(|i| mset.contains(i))
                    .collect();

                keys_got.iter().for_each(|i| {
                    let d = tree
                        .get(i)
                        .expect(&format!{"error with tree get id: {}", i});

                    segment_match.insert(
                        &format! {"{}", path_info::to_stmt(&d.path_info)},
                        &format! {r#"
                        {{
                            {scoped_plugin}
                            {func}(conn.clone());
                            return true;
                        }}
                        "#,
                            scoped_plugin = assign_plugin(d),
                            func = &d.func
                        }
                    );
                });

                if let Some(stmt) = segment_match.wrap("conn.path_info_match()[..]") {
                    segment_count_match.insert(seg_count, &stmt);
                };

                segment_match.clear();
            }

            for (l, lset) in static_length.set.iter() {
                keys_got = lset
                    .iter()
                    .map(|i| *i)
                    .filter(|i| mset.contains(i))
                    .collect();

                keys_got.iter().for_each(|i| {
                    let d = tree.get(i).expect(&format!{"error with tree get id: {}", i});
                    path_match.insert(
                        &format! {"{:?}", &d.path},
                        &format! {r#"
                        {{
                            {scoped_plugin}
                            {func}(conn.clone());
                            return true;
                        }}
                        "#,
                                  scoped_plugin = assign_plugin(d),
                                  func = &d.func
                        }
                    );
                });

                if let Some(stmt) = path_match.wrap("conn.path()") {
                    len_match.insert(l, &stmt);
                };

                path_match.clear();
            }

            let method = format!("Method::{}", method.to_uppercase());
            if let Some(expr) = len_match.wrap("conn.path().len()") {
                method_match.insert(&method, &expr);
            }
            if let Some(expr) = segment_count_match.wrap("conn.path_info().len()") {
                method_match_for_seg.insert(&method, &expr)
            }
            len_match.clear()
        };

        let stat =
            if let Some(expr) = method_match.wrap("conn.method()") {
                expr
            } else {
                "{}".to_string()
            };

        let dym =
            if let Some(expr) = method_match_for_seg.wrap("conn.method()") {
                expr
            } else {
                "{}".to_string()
            };

        (stat, dym)
    };

    let root = expanded.get(&1).unwrap();
    let code = format! {
        r#"
            const {name}: RouterType = |conn: Conn| {{
                {func};
                {func2};
                return false;
            }};
        "#,
        name = root.get_declr("const").expect("unable to find const keyword").1,
        func = static_finalized,
        func2 = dynamic_finalized
    };

    code.parse().unwrap()
}

#[derive(Default)]
struct IdCounter(usize);

impl IdCounter {
    fn incr(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[derive(Default)]
struct MakeMatch {
    pairs: Vec<String>
}

impl MakeMatch {
    fn insert(&mut self, key: &str, expr: &str) {
        let s = format!{r#"{} => {{
            {};
        }}"#, key, expr};
        self.pairs.push(s)
    }
    fn wrap(&self, target: &str) -> Option<String> {
        if self.pairs.len() == 0 {
            None
        } else {
            let s = format! { r#"
                match {} {{
                    {}
                    _ => {{ (); }}
                }}
            "#, target, self.pairs.join("\n")};
            Some(s)
        }
    }
    fn clear(&mut self) {
        self.pairs.clear();
    }
}

#[derive(Default, Debug)]
struct RelIndex {
    set: BTreeMap<String, BTreeSet<usize>>
}

impl RelIndex {
    fn insert(&mut self, key: impl ToString, id: usize) {
        let key_e = key.to_string();
        self.set.entry(key_e).or_default();
        self.set.get_mut(&key.to_string()).unwrap().insert(id);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum RegType {
    Methods,
    Resource,
    MethodName
}

#[derive(Debug)]
struct Dispatch {
    id: usize,
    path: String,
    func: String,
    method: String,
    reg_type: RegType,
    path_info: path_info::PathInfo,
    // TODO add keywords
    attachments: Vec<String>,
    pairs: Vec<(String, String)>
}

impl Dispatch {
    fn print(&self) {
        println!{
            "id: {} {} {}\nfunc: {}\nsegments: {:?}\n",
            self.id,
            self.method.to_uppercase(),
            self.path,
            self.func,
            self.path_info
        };
    }
    fn expand(self, c: &mut IdCounter) -> Vec<Self> {
        let mut vec = vec![];
        let resources = [
            ["get", "", "index"],
            ["get", "/new", "new"],
            ["post", "", "create"],
            ["get", "/show", "show"],
            ["post", "/:id/edit", "edit"],
            ["patch", "/:id", "update"],
            ["put", "/:id", "update"],
            ["delete", "", "destroy"]
        ];

        let func = |list: &[&str; 3]| {
            let [m, path, f] = list;
            let path = format!("{}{}", self.path, path);
            let path_info = path_info::compile_time_new(&path);

            let obj = Self {
                id: c.incr(),
                path,
                func: format!("{}::{}", self.func, f),
                method: m.to_string(),
                reg_type: RegType::MethodName,
                path_info,
                attachments: self.attachments.clone(),
                pairs: self.pairs.clone()
            };
            vec.push(obj);
        };
        match self.reg_type {
            RegType::Resource => {
                resources.iter().for_each(func);
            }
            _ => ()
        };
        vec
    }
}

#[derive(Clone, Debug)]
struct Scope {
    parent: usize,
    path: Vec<String>,
    // (keyword, path)
    declr_found: Vec<(String, String)>,
    nested_routes: Vec<(String, TokenStream)>
}

impl Scope {
    fn parse(path: Vec<String>, ts: TokenStream) -> Self {
        // temp values
        let mut keyword = String::new();
        let mut nested_routes = vec![];
        let mut declr_found = vec![];
        let mut temp = vec![];
        let mut scope_path = None;
        // state machine
        let mut searching_ident = true;
        let mut parsing_scope = false;
        let mut parsing_non_scope = false;

        for i in ts.into_iter() {
            // parsing logic here
            use TokenTree::*;
            if searching_ident {
                match i {
                    Ident(ident) => {
                        searching_ident = false;
                        if &ident.to_string() == "scope" {
                            parsing_scope = true;
                        } else {
                            keyword = ident.to_string();
                            parsing_non_scope = true;
                        }
                    }
                    _ => eprintln!("unexpected")
                }
            } else if parsing_scope {
                match i {
                    Literal(literal) if scope_path.is_none() => {
                        scope_path.replace(literal.to_string());
                    }
                    Group(g) => {
                        nested_routes.push((
                            parse_literal2str(scope_path.unwrap()),
                            g.stream()
                        ));
                        // refresh state
                        searching_ident = true;
                        parsing_scope = false;
                        scope_path = None;
                    }
                    _ => ()
                }
            } else if parsing_non_scope {
                match i {
                    Punct(p) if &p.to_string() == ";" => {
                        declr_found.push((
                            keyword.clone(),
                            temp.join("")
                        ));
                        temp.clear();

                        parsing_non_scope = false;
                        searching_ident = true;
                    }
                    any => {
                        temp.push(any.to_string());
                    }
                }
            } else {
                eprintln!("unexpected");
            }
        }

        Self {
            parent: 0,
            path,
            declr_found,
            nested_routes
        }
    }
    fn print(&self, id: &usize) {
        println!("id {} parent {} path: {:?}", id, self.parent, self.path);
    }
    fn get_declr(&self, key: impl ToString) -> Option<(&String, &String)> {
        let key = &key.to_string();
        for (k, v) in self.declr_found.iter() {
            if k == key {
                return Some((k, v))
            }
        }
        None
    }
}

fn parse_literal2str(l: impl ToString) -> String {
    l.to_string().replace('"', "")
}
