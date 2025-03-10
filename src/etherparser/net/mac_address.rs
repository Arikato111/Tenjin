use byteorder::WriteBytesExt;

#[derive(Clone, Copy)]
pub struct MacAddr {
    mac: [u8; 6],
}

impl MacAddr {
    pub fn new(mac: [u8; 6]) -> Self {
        Self { mac }
    }
}

impl MacAddr {
    pub fn to_string(&self) -> String {
        let mut mac_string = String::new();

        for m in self.mac.iter() {
            mac_string = format!("{}:{:02x}", mac_string, *m);
        }
        mac_string.remove(0);
        mac_string
    }
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        for m in self.mac.iter() {
            let _ = bytes.write_u8(*m);
        }
    }
}

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

impl From<[u8; 6]> for MacAddr {
    fn from(value: [u8; 6]) -> Self {
        Self { mac: value }
    }
}

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
