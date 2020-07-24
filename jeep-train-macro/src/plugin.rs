use crate::template;
use proc_macro::{TokenStream, TokenTree, Group, Delimiter};
use std::str::FromStr;
use std::iter::FromIterator;

pub fn main(input: TokenStream) -> TokenStream {
    // switch variables
    let mut switch_name = false;
    let mut switch_func = false;
    let mut switch_free = true;

    // data holders
    let mut temp  = vec![];
    let mut func_names = vec![];

    let mut const_name = vec![];

    input.into_iter().for_each(|i| {
        // this will try to find
        if switch_free {
            switch_free = false;
            match i.to_string().as_str() {
                "const" => switch_name = true,
                "func" => switch_func = true,
                _ => {
                    switch_free = true;
                    println!("Unexpected syntax")
                }
            }
        } else if switch_func {
            match i {
                TokenTree::Punct(e)
                if e.as_char() == ';' =>
                    {
                        switch_free = true;
                        switch_func = false;
                        func_names.push(temp.clone());
                        temp.clear();
                    }
                any => {
                    temp.push(any);
                }
            };
        } else if switch_name {
            match i {
                TokenTree::Punct(e) if e.as_char() == ';' => {
                    switch_free = true;
                    switch_name = false;
                    // const func declaration here
                    // contents are not inserted yet
                    const_name = temp.clone();
                    temp.clear();
                }
                any => {
                    temp.push(any);
                }
            }
        } else {
            println!("Unexpected syntax");
        };
    });

    let token_tree = {
        let mut temp = TokenStream::new();
        let mut index = 1;
        let not_halt = [1, func_names.len()-1];
        for func in func_names.drain(..) {
            index += 1;
            temp.extend(func);
            temp.extend(template::conn_call());
            if not_halt.contains(&index) {
                temp.extend(template::conn_halt());
            }
        }

        let group = Group::new(Delimiter::Brace, TokenStream::from_iter(temp));
        TokenStream::from(TokenTree::Group(group))
    };

    let plugin_type = TokenStream::from_str(": PluginType = |conn: Conn|").unwrap();
    let mut stream = TokenStream::from_str("const").unwrap();
    stream.extend(const_name);
    stream.extend(plugin_type);
    stream.extend(token_tree);
    stream.extend(TokenStream::from_str(";"));

    stream
}

