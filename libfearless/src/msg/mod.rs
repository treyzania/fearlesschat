mod plaintext;

use msg::plaintext::PlainMessage;

pub enum MessageData {
    Unknown(String),
    Plaintext(PlainMessage)
}

impl MessageData {

    pub fn get_spec_string(&self) -> &str {
        use msg::MessageData::*;
        match self {
            &Unknown(ref s) => s.as_str(),
            &Plaintext(_) => ".plain"
        }
    }

}

trait MessageType {

    fn from_string(data: &String) -> Self;
    fn to_string(&self) -> String;

}
