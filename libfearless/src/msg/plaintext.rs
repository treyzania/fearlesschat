#[derive(Serialize, Deserialize)]
pub struct PlainMessage {
    chan: String,
    msg: String
}

impl PlainMessage {

    pub fn new(chan: String, msg: String) -> PlainMessage {
        PlainMessage { chan: chan, msg: msg }
    }

}
