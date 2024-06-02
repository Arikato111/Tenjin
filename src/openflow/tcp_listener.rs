use crate::Controller;
use std::sync::{Arc, Mutex};
use std::{io::Read, net::TcpListener, thread};

use super::controller_frame::ControllerFrame;
use crate::openflow::ofp10::{traiter::OfpMsgEvent, HelloEvent};

pub fn tcp_listener_handler<OME: OfpMsgEvent>(address: &str) {
    let controller = Arc::new(Mutex::from(Controller::new()));
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let controller_clone = controller.clone();
                thread::spawn(move || {
                    let mut ctrl = match controller_clone.lock() {
                        Ok(guard) => guard,
                        Err(poinsoned) => {
                            let guard = poinsoned.into_inner();
                            guard
                        }
                    };
                    ctrl.send_msg(HelloEvent::new(), 0, &mut stream);
                    let ofp_size = ctrl.get_ofp().header_size();
                    // let ofp = controller.lock().unwrap().get_ofp();
                    let mut buffer = vec![0u8; ofp_size];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(v) if v > 0 => {
                                ctrl.request_handler(&mut buffer, &mut stream);
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
