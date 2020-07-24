use serde_json;

type Json = serde_json::Value;
type Map = serde_json::Map<String, Json>;

pub struct PathElm {
    pub segment: String,
    pub is_param: bool
}

pub enum ReturnType {
    String(Vec<String>),
    PathElm(Vec<Vec<PathElm>>)
}

/// returns a paths for matching;
pub fn benchmark_dataset() -> Vec<String> {
    if let ReturnType::String(vec) = arango_keys(false) {
        return vec
    } else {
        unreachable!()
    };
}

/// set false if you want a string
/// if you want the path to be a vector of PathElm, set true
pub fn arango_keys(return_type_pathelm: bool) -> ReturnType {
    let file = std::fs::read_to_string("../dataset/arangodb.json").unwrap();
    let json: Json = serde_json::from_str(&file).unwrap();
    let map: &Map = json.as_object().unwrap();
    let map: &Map = map.get("paths").unwrap().as_object().unwrap();

    let keys = {
        let vec = map.iter().map(|(key, _)|
            key
                .to_string()
                .replacen("-", "_", 999)
        ).collect::<Vec<String>>();
        if return_type_pathelm == false {
           return ReturnType::String(vec)
        };
        vec
    };


    let vec = keys.iter().map(|key| {
        let seg: Vec<&str> = key.split("/").collect();

        seg.iter().map(|s| {
            PathElm {
                is_param: s.contains("{"),
                segment: s.to_string().replacen("-", "_", 888)
            }
        }).collect::<Vec<PathElm>>()

    }).collect::<Vec<Vec<PathElm>>>();

    ReturnType::PathElm(vec)
}
