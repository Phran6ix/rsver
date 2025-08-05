use std::{
    error::Error,
    hash::RandomState,
    io,
    net::{IpAddr, Ipv4Addr},
};

use crate::dns::question;

pub fn get_answer(response_bytes: &[u8]) -> Result<String, io::Error> {
    let len_of_response = response_bytes.len();

    // let flags = u16::from_le_bytes([response_bytes[2], response_bytes[3]]);
    // println!("FLAGS =>{flags} ");

    let flags_hi = response_bytes[2];
    let flags_lo = response_bytes[3];

    let qr = (flags_hi & 0b1000_0000) >> 7;
    if qr == 0 {
        panic!("Answer does not contain Answer");
    }

    //Zero cos we terminate the QNAME SECTION with 0
    // let us confirm that the there is no error in our req

    let rcode_byte = (flags_lo & 0x000F) as u8;

    // if rcode_byte != 0 {
    match rcode_byte {
        0 => {
            println!("QuestionQuery is valid. ")
        }
        1 => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Format error - The name server was unable to interpret the query.",
            ));
        }
        2 => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Format error - The name server was unable to process this query due to a problem with the name server..",
            ));
        }
        3 => {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                " Error - Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist. ",
            ));
        }
        code @ 4..15 => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("unknown dns error {}", code),
            ));
        }
        _ => unreachable!(),
    }
    // }

    let mut qname_offset = 12;

    // println!(
    //     "initial qname_offset = {} The Question => {:?}",
    //     response_bytes[qname_offset],
    //     &response_bytes[qname_offset..]
    // );

    let mut original_url: String = String::new();

    while response_bytes[qname_offset] != 0 {
        let len = response_bytes[qname_offset] as usize;

        // from the start of the qname + the word length + 1(to move from last byte of the
        // word to the start of the next - which is the length of the next word)

        let word_byte: &[u8] = &response_bytes[qname_offset..qname_offset + len + 1];

        let word = std::str::from_utf8(word_byte).expect("Cannot parse this word byte");
        original_url.push_str(word);
        original_url.push_str(".");

        qname_offset += 1 + len;
    }

    println!("Original URL: {} \n\n", original_url);

    let mut answer_offset = qname_offset + 1 + 2 + 2;

    // RESOURCE(ANSWER BYTE ALLOCATION)
    // NAME = X bits end by 0
    // TYPE = 16 bits
    // CLASS = 16 bits
    // TTL = 32 bits
    // RDLENGTH = 16 bits
    // RDATA = if type = A and class = IN , then RDATA = 32
    //
    // TO SKIP OVER THE NAME FIELD - SINCE THE NAME CAN BE APOINTER TO THE QNAME OR THE DMAIN NAME
    // - Let handle each case

    if response_bytes[answer_offset] & 0b1100_0000 == 0b1100_0000 {
        answer_offset += 2;
    } else {
        while response_bytes[answer_offset] != 0 {
            let len = response_bytes[answer_offset] as usize;
            answer_offset += len + 1;
        }
        answer_offset += 1;
    }

    answer_offset += 2;
    answer_offset += 2;
    answer_offset += 4;

    let rlength = u16::from_be_bytes([
        response_bytes[answer_offset],
        response_bytes[answer_offset + 1],
    ]);

    // println!("rlength, {rlength}");
    answer_offset += 2;

    // let mut domain_ip: String = String::new();
    //
    // for i in (1..rlength + 1) {
    //     println!("ITERA , {i}");
    //     domain_ip.push_str(&format!("{}", response_bytes[answer_offset]));
    //     if i < rlength {
    //         domain_ip.push_str(".");
    //     }
    //     answer_offset += 1;
    // }
    //

    let domain_ip = Ipv4Addr::new(
        response_bytes[answer_offset],
        response_bytes[answer_offset + 1],
        response_bytes[answer_offset + 2],
        response_bytes[answer_offset + 3],
    );

    println!("THE DOMAIN IP = {domain_ip}");

    Ok(String::new())
}
