use crate::ofp_from_version;
use crate::openflow::{messages::Openflow10, traiter::OfpMsgEvent, Controller};
use std::{io::Read, net::TcpListener, thread};

pub fn tcp_listener_handler(ofp_version: u8, address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    /*
                     * when spawn new thread.
                     * The Controller will be create.
                     */
                    let mut controller = Controller::new(ofp_from_version!(ofp_version));
                    controller.hello(&mut stream);
                    let mut buffer = vec![0u8; controller.ofp.header_size()];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(v) if v > 0 => {
                                controller.request_handler(&mut buffer, &mut stream);
                            }
                            Ok(_) | Err(_) => break,
                        }
                    }
                });
            }
            Err(_) => panic!("Connection failed!"),
        }
    }
}
