extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct MsgContainer {
    version: String,
    spec: String,
    contents: String
}

trait MessageContents {

    fn from_string(raw: &String) -> Self;

}
