// use byteorder::{BigEndian, WriteBytesExt};
//
// use std::io::Write;
pub struct Header {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    pub fn construct() -> Self {
        Self {
            id: 22,
            flags: 0b0000_0001_0000_0000,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        // let mut cursor = &mut buf[..];

        // (&mut buf[..]).write_u16(buf)
        buf[0] = (self.id >> 8) as u8;
        buf[1] = self.id as u8;

        buf[2] = (self.flags >> 8) as u8;
        buf[3] = self.flags as u8;

        buf[4] = (self.qdcount >> 8) as u8;
        buf[5] = self.qdcount as u8;

        buf[6] = (self.ancount >> 8) as u8;
        buf[7] = self.ancount as u8;

        buf[8] = (self.nscount >> 8) as u8;
        buf[9] = self.nscount as u8;

        // ALTERNATIVE TO BYTE SHIFT - THE RUST WAY
        let [hi, lo] = self.arcount.to_be_bytes();
        buf[10] = hi;
        buf[11] = lo;

        buf
    }
}
