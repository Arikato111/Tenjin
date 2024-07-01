use byteorder::WriteBytesExt;

#[derive(Clone)]
pub struct MacAddr {
    mac: [u8; 6],
}

impl MacAddr {
    pub fn new(mac: [u8; 6]) -> Self {
        Self { mac }
    }
}

impl MacAddr {
    pub fn marshal(&self, bytes: &mut Vec<u8>) {
        for m in self.mac.iter() {
            bytes.write_u8(*m);
        }
    }
}

impl From<MacAddr> for u64 {
    fn from(value: MacAddr) -> Self {
        let mut byte: u64 = 0;
        for i in 0..6 {
            byte = byte << 8;
            byte += value.mac[i] as u64;
        }
        byte
    }
}

impl From<u64> for MacAddr {
    fn from(value: u64) -> Self {
        let mut mac = [0u8; 6];
        for i in 0..6 {
            mac[i] = (value >> (i * 8)) as u8;
        }
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
