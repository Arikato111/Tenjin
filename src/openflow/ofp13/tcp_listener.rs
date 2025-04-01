use super::{ControllerFrame13, OfpMsgEvent};
use crate::openflow::ofp13::HelloEvent;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

/// Handles incoming TCP connections for OpenFlow 1.3 protocol
///
/// # Arguments
/// * `address` - The address to bind the TCP listener to (e.g., "127.0.0.1:6633")
/// * `controller` - The OpenFlow controller implementation that will handle the connections
///
/// # Returns
/// * `Result<(), std::io::Error>` - Returns Ok(()) if successful, or an IO error if binding fails
pub async fn tcp_listener_handler(
    address: &str,
    controller: &(impl ControllerFrame13 + 'static + Clone + Sync),
) -> Result<(), std::io::Error> {
    // Bind to the specified address and start listening for connections
    let listener = TcpListener::bind(address).await?;

    // Continuously accept new connections
    loop {
        let (mut stream, _) = listener.accept().await?;
        if let Ok(addr) = stream.peer_addr() {
            println!("server has connection from {}", addr);
        }

        // Clone the controller for this connection and spawn a new task to handle it
        let mut ctrl = controller.clone();
        tokio::spawn(async move {
            processing(&mut ctrl, &mut stream).await;
        });
    }
}

/// Processes individual TCP connections for OpenFlow 1.3 protocol
///
/// # Arguments
/// * `ctrl` - The OpenFlow controller instance handling this connection
/// * `stream` - The TCP stream for this connection
async fn processing(ctrl: &mut (impl ControllerFrame13 + Clone + Sync), stream: &mut TcpStream) {
    // Send initial OpenFlow HELLO message to establish protocol version
    ctrl.send_msg(HelloEvent::new(), 0, stream).await;

    // Get the size of OpenFlow header and prepare buffer for reading messages
    let ofp_size = ctrl.ofp().header_size();
    let mut buffer = vec![0u8; ofp_size];

    // Main message processing loop
    loop {
        match stream.read(&mut buffer).await {
            Ok(v) if v > 0 => {
                // Process the received OpenFlow message
                ctrl.request_handler(&mut buffer, stream).await;
            }
            Ok(_) => {
                // Connection closed by peer
                break;
            }
            Err(_) => {
                println!("cannot read packet");
                break;
            }
        }
    }
}
