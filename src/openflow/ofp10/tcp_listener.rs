use super::{ControllerFrame10, OfpMsgEvent};
use crate::openflow::ofp10::HelloEvent;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

/// Handles incoming TCP connections for OpenFlow v1.0 protocol
/// 
/// # Arguments
/// * `address` - The address to bind the TCP listener to (e.g. "127.0.0.1:6633")
/// * `controller` - The OpenFlow controller implementation that will handle the connections
/// 
/// # Returns
/// * `Result<(), std::io::Error>` - Returns Ok(()) if successful, or an IO error if binding fails
pub async fn tcp_listener_handler(
    address: &str,
    controller: &(impl ControllerFrame10 + 'static + Clone + Sync),
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

/// Processes individual TCP connections for OpenFlow v1.0 protocol
/// 
/// # Arguments
/// * `ctrl` - The OpenFlow controller implementation
/// * `stream` - The TCP stream for the connection
async fn processing(ctrl: &mut (impl ControllerFrame10 + Clone + Sync), stream: &mut TcpStream) {
    // Send initial Hello message to establish the connection
    ctrl.send_msg(HelloEvent::new(), 0, stream).await;
    
    // Get the size of OpenFlow header and create a buffer for reading messages
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
