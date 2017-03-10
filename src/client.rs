extern crate protobuf;
extern crate zmq;

mod note_stream;
mod messages;

use protobuf::Message;
use messages::ping::Ping;
use messages::note::{Note, Note_Pitch};

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUSH).unwrap();
    socket.connect("tcp://127.0.0.1:1371").unwrap();

    let mut note = Note::new();
    let mut ping = Ping::new();
    ping.set_message("PING".to_string());
    let encoded_ping = ping.write_to_bytes().unwrap();

    note.set_pitch(Note_Pitch::PING);
    note.set_body(encoded_ping);
    let encoded_note = note.write_to_bytes().unwrap();

    loop {
        socket.send(&encoded_note, 0).unwrap();
    }
}