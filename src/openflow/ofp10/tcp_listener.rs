use super::{ControllerFrame10, OfpMsgEvent};
use crate::openflow::ofp10::HelloEvent;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub async fn tcp_listener_handler(
    address: &str,
    controller: &(impl ControllerFrame10 + 'static + Clone + Sync),
) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(address).await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        if let Ok(addr) = stream.peer_addr() {
            println!("server has connection from {}", addr);
        }

        let mut ctrl = controller.clone();
        tokio::spawn(async move {
            processing(&mut ctrl, &mut stream).await;
        });
    }
}

async fn processing(ctrl: &mut (impl ControllerFrame10 + Clone + Sync), stream: &mut TcpStream) {
    ctrl.send_msg(HelloEvent::new(), 0, stream).await;
    let ofp_size = ctrl.ofp().header_size();
    let mut buffer = vec![0u8; ofp_size];
    loop {
        match stream.read(&mut buffer).await {
            Ok(v) if v > 0 => {
                ctrl.request_handler(&mut buffer, stream).await;
            }
            Ok(_) => {
                break;
            }
            Err(_) => {
                println!("cannot read packet");
                break;
            }
        }
    }
}
