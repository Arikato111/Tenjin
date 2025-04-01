use byteorder::WriteBytesExt;

/// A structure for managing MAC (Media Access Control) addresses.
/// 
/// This struct provides functionality for creating, converting, and manipulating MAC addresses.
/// It stores the MAC address as a 6-byte array and provides various conversion methods.
#[derive(Clone, Copy, Debug)]
pub struct MacAddr {
    mac: [u8; 6],
}

impl MacAddr {
    /// Creates a new MAC address from a 6-byte array.
    /// 
    /// # Arguments
    /// * `mac` - A 6-byte array representing the MAC address
    pub fn new(mac: [u8; 6]) -> Self {
        Self { mac }
    }

    /// Converts the MAC address to a human-readable string format (XX:XX:XX:XX:XX:XX).
    /// 
    /// # Returns
    /// A string representation of the MAC address in hexadecimal format
    pub fn to_string(&self) -> String {
        let mut mac_string = String::new();

        for m in self.mac.iter() {
            mac_string = format!("{}:{:02x}", mac_string, *m);
        }
        mac_string.remove(0);
        mac_string
    }

    /// Serializes the MAC address into a byte vector.
    /// 
    /// # Arguments
    /// * `bytes` - The vector to append the MAC address bytes to
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        for m in self.mac.iter() {
            let _ = bytes.write_u8(*m);
        }
    }
}

/// Converts a MacAddr into a u64 representation.
/// The bytes are packed into the u64 with the first byte in the most significant position.
impl From<MacAddr> for u64 {
    fn from(value: MacAddr) -> Self {
        let mut byte: u64 = 0;
        for m in value.mac.iter() {
            byte = byte << 8;
            byte += *m as u64;
        }
        byte
    }
}

/// Creates a MacAddr from a 6-byte array.
impl From<[u8; 6]> for MacAddr {
    fn from(value: [u8; 6]) -> Self {
        Self { mac: value }
    }
}

/// Creates a MacAddr from a u64 value.
/// The bytes are unpacked from the u64 with the first byte from the most significant position.
impl From<u64> for MacAddr {
    fn from(value: u64) -> Self {
        let mut mac = [0u8; 6];
        for i in 0..6 {
            mac[i] = (value >> (i * 8)) as u8;
        }
        mac.reverse();
        Self { mac }
    }
}

/// Creates a MacAddr from a string representation.
/// 
/// # Arguments
/// * `value` - A string in the format "XX:XX:XX:XX:XX:XX" where XX are hexadecimal values
/// 
/// # Returns
/// A new MacAddr instance. If parsing fails, the address will be set to 00:00:00:00:00:00
impl From<&str> for MacAddr {
    fn from(value: &str) -> Self {
        let mac_vec = value
            .split(":")
            .map(|x| match u8::from_str_radix(x, 16) {
                Ok(v) => v,
                Err(_) => 0,
            })
            .collect::<Vec<u8>>();
        let mut mac = [0u8; 6];
        for i in 0..6 {
            mac[i] = mac_vec[i];
        }
        Self { mac }
    }
}
