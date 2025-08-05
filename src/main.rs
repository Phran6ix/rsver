use hex;
use std::env;

mod dns;
mod udp;

use dns::header::Header;
use udp::connection;

use crate::dns::{answer, question};

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain_name: &str;

    match args.get(1) {
        Some(d) => domain_name = d.as_str(),
        None => panic!("Expecting a domain name in input"),
    }

    let headers = Header::construct();

    let mut question_query = question::get_query_question(domain_name);
    // let mut question_query = question::query_bytes(&question);
    //RESERVE FOURMORE BYTES FOR QTYPE AND QCLASS

    question_query.reserve(4);

    let q_type: [u8; 2] = 1u16.to_be_bytes();
    let q_class: [u8; 2] = 1u16.to_be_bytes();

    question_query.extend_from_slice(&q_type);
    question_query.extend_from_slice(&q_class);

    let mut question_msg_bytes = headers.to_bytes().to_vec();
    question_msg_bytes.extend_from_slice(&question_query);

    // NATTIVE WAY OF CONVERTING TO HEX
    // let message_hex_2 = question_msg_bytes
    //     .iter()
    //     .map(|b| format!("{:02x}", b))
    //     .collect::<Vec<String>>()
    //     .join("");
    // println!("THE HEX USING format => {}", message_hex_2);

    // let message_hex = hex::encode(&question_msg_bytes);

    //INITIALIZE CONNECTION
    // let addr = "127.0.0.1:34254";
    let addr = "0.0.0.0:0";
    let socket = connection::connect(addr).unwrap();
    println!("Connection ready on : {:?}", socket);

    // WE SHALL SEND THE MESSAGE
    connection::send_msg(&socket, &question_msg_bytes).unwrap();

    // LISTEN FOR MESSAAGE
    let buf = connection::receive_msg(&socket).unwrap();
    // let buf_hex = hex::encode(&buf);
    answer::get_answer(&buf).unwrap();
}
