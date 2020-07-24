use proc_macro::TokenStream;
use std::iter::FromIterator;
use std::str::FromStr;

/// returns a declaration of router macro
#[proc_macro]
pub fn main(_ts: TokenStream) -> TokenStream {
    let elm = match dataset::arango_keys(true) {
        dataset::ReturnType::PathElm(elm) => elm,
        _ => unreachable!()
    };

    let mut vector = Vec::with_capacity(elm.len());

    let string = r#"
        router!{
            const BENCHMARK_ROUTER;
    "#.to_string();
    vector.push(string);
    
    for e in elm.iter() {
        let mut temp = "".to_owned();

        for elm in e.iter() {
            let s = if elm.is_param {
                format!("/:{}", &elm.segment)
            } else {
                elm.segment.to_owned()
            };
            temp.push_str(s.as_str())
        }

        let string = format!(r#"
            scope {:?} {{
                get benchmark_func;
            }}
        "#, temp);

        vector.push(string)
    }
    vector.push("}".to_string());

    TokenStream::from_str(&vector.join("\n")).unwrap()
}