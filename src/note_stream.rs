extern crate zmq;
extern crate protobuf;

use messages::note::{Note as NoteMessage, Note_Pitch};
use messages::ping::Ping as PingMessage;

pub enum Note {
    Unknown,
    Ping(PingMessage)
}

pub struct NoteStream {
    socket: zmq::Socket,
    message: zmq::Message
}

impl NoteStream {
    pub fn connect(ctx: zmq::Context, address: &str) -> NoteStream {
        let socket = ctx.socket(zmq::PULL).expect("make socket");
        socket.bind(address).expect("bind note stream");
        NoteStream {
            socket: socket,
            message: zmq::Message::new().unwrap()
        }
    }
}

impl Iterator for NoteStream {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        self.socket.recv(&mut self.message, 0).expect("receive message");
        let note: NoteMessage = protobuf::parse_from_bytes(&*self.message).unwrap();

        match note.get_pitch() {
            Note_Pitch::PING => {
                let message: PingMessage = protobuf::parse_from_bytes(note.get_body()).unwrap();
                Some(Note::Ping(message))
            }
        }
    }
}
