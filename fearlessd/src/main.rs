use std::io;
use std::str;
use std::String;
use bytes::BytesMut;

extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
use tokio_io::codec::{Encoder, Decoder};

extern crate libfearless;
use libfearless::net::MsgContainer;

fn main() {
    println!("Hello, world!");
}

pub struct FlchatCodec;

impl Decoder for FlchatCodec {

    type Item = MsgContainer;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<MsgContainer>> {

        if let Some(i) = buf.iter().position(|&b| b == b'\n') {

            let line = buf.split_to(i);
            buf.split_to(1); // To remove the newline at the end.

            match str::from_utf8(&line) {
                Ok(s) => Ok(MsgContainer::from_json(String::from_str(s))),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }

        }

    }

}

impl Encoder for FlchatCodec {

    type Item = MsgContainer;
    type Error = io::Error;

    fn encode(&mut self, msg: MsgContainer, buf: &mut BytesMut) -> io::Result<()> {

        buf.extend(msg.to_string().bytes());
        buf.extend(b"\n");
        Ok(())

    }

}
