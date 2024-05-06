use std::{io::Read, net::TcpListener};

use crate::openflow::{traiter::OfpMsgEvent, Controller};

pub fn tcp_listener_handler<OME: OfpMsgEvent>(controller: &mut Controller<OME>, address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    /*
     * buffer with 8 length that only support openflow 1.0
     * I will make it support others version soon.
     */
    let mut buffer = vec![0u8; 8];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                controller.hello(&mut stream);
                loop {
                    match stream.read(&mut buffer) {
                        Ok(v) if v > 0 => {
                            controller.request_handler(&mut buffer, &mut stream);
                        }
                        Ok(_) | Err(_) => panic!("Connection failed!"),
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
