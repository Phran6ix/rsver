use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

pub fn connect(addr: &str) -> Result<UdpSocket, io::Error> {
    let local_addr = addr
        .parse::<SocketAddr>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input address"))?;

    let socket = UdpSocket::bind(local_addr).expect("Error occured when binding address");
    println!("SOCKET AD {}", socket.local_addr().unwrap());

    let remote_address = "8.8.8.8:53"
        .parse::<SocketAddr>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid remote address input"))?;

    println!("Connecting to: {}", remote_address);
    socket.connect(remote_address)?;
    // .expect("Could not connect to 8.8..");

    Ok(socket)
}

pub fn send_msg(socket: &UdpSocket, msg: &[u8]) -> Result<(), io::Error> {
    socket.send(msg).expect("An error occured when sending msg");

    Ok(())
}

pub fn receive_msg(socket: &UdpSocket) -> Result<[u8; 512], io::Error> {
    let mut buf = [0; 512];
    println!("BOUT TO LISTEN TO MSG");
    // let (_, src_address) = socket.recv_from(&mut buf).expect("No message received");

    match socket.recv(&mut buf) {
        Ok(received) => println!("This is the recieved bytes {:?}", &buf[..received]),
        Err(e) => println!("An error occured while trying to receive => {:?}", e),
    }
    // println!("RECIEVED DATAGRAM FROM {src_address}");

    // let filled_buf = &mut buf[..num_of_bytes];
    Ok(buf)
}
