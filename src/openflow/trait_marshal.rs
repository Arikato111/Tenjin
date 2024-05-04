use super::OfpMsg;

/**
 * the trait for parse value to bytes.
 * for use with Controller's send_msg.
 */
pub trait MessageMarshal {
    fn marshal(&self, bytes: &mut Vec<u8>);
    fn msg_code(&self) -> OfpMsg;
    fn size_of(&self) -> usize;
}
