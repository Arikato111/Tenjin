use tenjin::openflow::{Controller, Msg, OfpHeader};
use std::io::Read;
use std::net::TcpListener;

extern crate byteorder;

fn main() -> Result<(), std::io::Error> {
    let controller = Controller::new(Controller::OFP_1_0);

    let listener = TcpListener::bind(("127.0.0.1", 6633)).unwrap();
    let mut buf = vec![0u8; 8];
    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                // std::thread::spawn(move || {
                println!("=================== connection =======================");

                // after tcp is connected, it will send hello message
                controller.hello(&mut stream);

                // loop for receive data
                loop {
                    // first receive with Openflow header 64 bit to buf
                    match stream.read(&mut buf) {
                        Ok(v) if v > 0 => {
                            let packet = OfpHeader::parse(&buf);
                            // length_payload is var to receive payload if the packet has
                            // and assign size by length
                            let length_payload = packet.size();
                            let mut payload = vec![0u8; length_payload];
                            stream.read(&mut payload)?;
                            let message = Msg::parse(packet.message, &payload);

                            match message {
                                // 0 is Hello message
                                Msg::Hello => {
                                    // after get Hello, send fetureReq
                                    controller.feture_req(packet.xid, &mut stream);
                                    println!("Hello event");
                                }
                                Msg::PacketIn(b) => {
                                    controller.packetIn(xid, &payload, &mut stream);
                                    println!("PacketIn event");
                                }
                                _ => {
                                    println!("others message");
                                }
                            }
                        }
                        Ok(_) | Err(_) => break,
                    }
                }
                println!("======================================================");

                // });
            }
            Err(_) => {
                // connection failed
                panic!("Connection failed")
            }
        }
    }
    Ok(())
}
