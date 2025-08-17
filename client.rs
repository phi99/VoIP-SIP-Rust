#In progress
use rvoip_sip_core::{Method, Request, Response, StatusCode, Uri};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr: SocketAddr = "127.0.0.1:5060".parse()?;
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("SIP Client sending request to {}", server_addr);

    let uri = Uri::from("sip:server.com"); // Example URI
    let method = Method::Register; 

    let request = Request::new(method, uri);

    let request_bytes = request.to_string().into_bytes();

    socket.send_to(&request_bytes, server_addr).await?;
    println!("Sent REGISTER request to server");

    let mut buf = [0; 1024];

    let (len, addr) = socket.recv_from(&mut buf).await?;
    let response_str = str::from_utf8(&buf[..len]).unwrap();
    println!("Received response from {}: \n{}", addr, response_str);

    let response = Response::new(StatusCode::Ok);  // Default to Ok (you could enhance this)

    if response.status_code() == 200 {  // 200 corresponds to StatusCode::Ok
        println!("SIP response: 200 OK");
    } else {
        println!("SIP response: {}", response.status_code());
    }

    Ok(())
}
