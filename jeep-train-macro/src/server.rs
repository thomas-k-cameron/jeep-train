use crate::template;
use proc_macro::{TokenStream, TokenTree, Group, Delimiter};
use std::iter::FromIterator;
use std::str::FromStr;

pub fn main(input: TokenStream) -> TokenStream {
    // paris_found. This will be returned from this block
    let mut pairs_found = {
        let stream = input.clone();
        let mut pairs_found = vec![];
        // temporary variables
        let mut keyword: Option<TokenTree> = None;
        let mut temp = vec![];

        for tt in stream.into_iter() {
            match tt {
                // push the stuff found on pairs_found and refresh the state
                TokenTree::Punct(c) if c.as_char() == ';' => {
                    pairs_found.push((keyword.unwrap(), temp.clone()));

                    keyword = None;
                    temp.clear();
                }
                any => {
                    if keyword.is_none() {
                        keyword.replace(any);
                    } else {
                        temp.push(any);
                    }
                }
            }
        }

        pairs_found
    };

    let (name, content) = {
        let mut name = None;
        let mut content = vec![];

        for (key, arg) in pairs_found.drain(..) {
            match key.to_string().as_str() {
                "plugin" => {
                    let mut path = TokenStream::from_iter(arg);
                    path.extend(template::conn_call());
                    path.extend(template::conn_halt());
                    content.push(path)
                },
                "router" => {
                    let iter = TokenStream::from_iter(arg);
                    let code = format!("if {}(conn.clone()) {{ return; }}", iter.to_string());
                    content.push(code.parse().unwrap());
                }
                "fn" => name = Some(arg),
                _ => panic!("unexpected key: {}", key)
            }
        }
        (name.unwrap(), content)
    };

    let mut stream = TokenStream::from_str("fn").unwrap();
    stream.extend(name);
    stream.extend(TokenStream::from_str("(conn: Conn)"));
    stream.extend(vec![
        TokenTree::Group(
            Group::new(Delimiter::Brace, TokenStream::from_iter(content))
        )
    ]);

    println!("{}", stream.to_string());

    stream
}

