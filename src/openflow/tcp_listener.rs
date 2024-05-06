use crate::openflow::{messages::Openflow10, traiter::OfpMsgEvent};
use crate::{ofp_from_version, Controller};
use std::sync::{Arc, Mutex};
use std::{io::Read, net::TcpListener, thread};

use super::controller_frame::ControllerFrame;
use super::events::HelloEvent;

pub fn tcp_listener_handler<OME: OfpMsgEvent>(address: &str, ofp_version: u8) {
    let controller = Arc::new(Mutex::from(Controller::new(ofp_from_version!(ofp_version))));
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let controller_clone = controller.clone();
                thread::spawn(move || {
                    controller_clone
                        .lock()
                        .unwrap()
                        .send_msg(HelloEvent::new(), 0, &mut stream);
                    let ofp_size = controller_clone.lock().unwrap().get_ofp().header_size();
                    // let ofp = controller.lock().unwrap().get_ofp();
                    let mut buffer = vec![0u8; ofp_size];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(v) if v > 0 => {
                                controller_clone
                                    .lock()
                                    .unwrap()
                                    .request_handler(&mut buffer, &mut stream);
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
