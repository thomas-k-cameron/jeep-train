pub type PathInfo = Vec<(PathElm, String)>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PathElm {
    Static,
    Dynamic
}

pub fn runtime_new(s: &str) -> Vec<String> {
    let mut vec = Vec::with_capacity(32);
    for i in s.split("/") {
        vec.push(i.to_string())
    }
    vec
}

pub fn compile_time_new(s: &str) -> PathInfo {
    let mut f: Vec<String> = s.split("/")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| String::from(*s))
        .collect();

    let mut vec = Vec::with_capacity(36);
    for mut s in f.drain(..) {
        let elm = if let Some(':') = s.chars().nth(0) {
            s.replace_range(0..1, "");
            PathElm::Dynamic
        } else {
            PathElm::Static
        };
        vec.push((elm, s));
    }
    vec
}

pub fn to_stmt(o: &PathInfo) -> String {
    let mut content = vec![];
    for (e, s) in o.iter() {
        let s = match e {
            PathElm::Static => {
                format!{"{:?}", s.to_owned()}
            }
            PathElm::Dynamic => {
                "_".to_owned()
            }
        };
        content.push(s)
    }

    format!("[ {} ]", content.join(","))

}