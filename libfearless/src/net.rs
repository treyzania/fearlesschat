extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct MsgContainer {
    version: String,
    spec: String,
    contents: String
}

impl MsgContainer {

    pub fn from_json(json: &String) -> Option<MsgContainer> {
        serde_json::from_str(json).unwrap_or(None)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}
