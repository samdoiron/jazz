extern crate protobuf;
extern crate zmq;

mod messages;
mod note_stream;

use note_stream::{NoteStream, Note};

fn main() {
    let ctx = zmq::Context::new();
    let stream = NoteStream::connect(ctx, "tcp://127.0.0.1:1371");
    for note in stream {
        match note {
            Note::Unknown => {}
            Note::Ping(ping) => {
                println!("got ping {}", ping.get_message());
            }
        }
    }
}