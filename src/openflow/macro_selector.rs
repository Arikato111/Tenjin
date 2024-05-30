/**
 * this macro use for create Controller with version inside thread.
 * Tt is used at 'tcp_listener/tcp_listener_handler.rs'
 */
#[macro_export]
macro_rules! ofp_from_version {
    ($ofp_version: expr) => {
        match $ofp_version {
            1 => Openflow10::new(),
            _ => panic!("This version is not support")
        }
    };
}