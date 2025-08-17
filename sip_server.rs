#In progress
    
use rvoip_sip_core::{Request, Response, StatusCode, Method, Uri};
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr: SocketAddr = "0.0.0.0:5060".parse()?;
    let socket = UdpSocket::bind(&server_addr).await?;
    println!("SIP Server listening on {}", server_addr);

    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message = str::from_utf8(&buf[..len]).unwrap();
        println!("Received SIP request: \n{}", message);

        let first_line = message.lines().next().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        if parts.len() < 3 {
            println!("Invalid SIP request: missing parts.");
            continue;
        }

        let method = parts[0];  
        let uri = parts[1];     

        let method_enum = match method {
            "REGISTER" => Method::Register,
            "INVITE" => Method::Invite,
            "BYE" => Method::Bye,
            "ACK" => Method::Ack,
            "CANCEL" => Method::Cancel,
            "OPTIONS" => Method::Options,
            _ => {
                println!("Unsupported method: {}", method);
                continue;
            }
        };

        let uri_obj = Uri::from(uri);  // Uri::from() takes &str, not String

        let request = Request::new(method_enum, uri_obj);

        match request.method() {
            Method::Register => {
                println!("Received REGISTER request");
                let response = Response::new(StatusCode::Ok);
                let response_str = response.to_string();
                socket.send_to(response_str.as_bytes(), addr).await?;
                println!("Sent 200 OK response");
            }
            _ => {
                println!("Received unhandled method: {:?}", request.method());
            }
        }
    }
}

