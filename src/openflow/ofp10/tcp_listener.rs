use crate::Controller;
use std::{io::Read, net::TcpListener, thread};

use crate::openflow::ofp10::HelloEvent;

use super::{ControllerFrame10, OfpMsgEvent};

pub fn tcp_listener_handler(address: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Ok(addr) = stream.peer_addr() {
                    println!("server has connection from {}", addr);
                }

                thread::spawn(move || {
                    let mut ctrl = Controller::new();
                    ctrl.send_msg(HelloEvent::new(), 0, &mut stream);
                    let ofp_size = ctrl.ofp().header_size();
                    let mut buffer = vec![0u8; ofp_size];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(v) if v > 0 => {
                                ctrl.request_handler(&mut buffer, &mut stream);
                            }
                            Ok(_) => {
                                continue;
                            }
                            Err(_) => {
                                println!("cannot read packet");
                                break;
                            }
                        }
                    }
                });
            }
            Err(_) => panic!("Connection failed!"),
        }
    }
    Ok(())
}
