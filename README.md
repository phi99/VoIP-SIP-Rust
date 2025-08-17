***In Progress**

	 	---------------------------------------------------------------------------------------------------
	     	                   ** Implementing VoIP/SIP systems with Rust **
	  	---------------------------------------------------------------------------------------------------


```text

                            Basic VoIP Traffic Flow Overview
                           ----------------------------------

                                        +---------------+			
[Client A]                              |   SIP Server  |                                 [Client B]
    |                                   |(Call Manager) |          			                   |	
    |                                   +---------------+                                      |
	|        				                     |						                       |
	|	       SIP REGISTER/OK			         | 		          SIP REGISTER/OK			   |	
	|<--------------------------------------->   |  <--------------------------------------->  |
	|                                            |                                             |
    |    1. SIP INVITE (Call Initiation)         |                                             |
    | -----------------------------------------> |                                             |
    |                                            |    2. Forward SIP INVITE to Client B        |
    |                                            | ------------------------------------------->|
    |                                            |         SIP 200 OK (Call Accepted)          |
    |    3. SIP 200 OK (Call Accepted)           | <-----------------------------------------  |
    | <----------------------------------------- |                                             |
    |                                            |                                             |
    |    4. SIP ACK (Call Confirmed)             |                                             |
    | ------------------------------------------>|                                             |
    |                                            |                                             |
    |                                            |                                             |
    |                                            |                                             |
    |   5. Start Media Exchange (RTP)     +---------------+   5. Start Media Exchange (RTP)    |
    |<----------------------------------->| Media Server  | <--------------------------------->|               
    |        (RTP Audio Stream)           | (Handles RTP) |      (RTP Audio Stream)            |
    |                                     +---------------+   				                   |
    |            6. SIP BYE        		         |					6. SIP BYE                 |
	| <----------------------------------->	[SIP server] <-----------------------------------> |


```

```text
SIP Client
-----------
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

```

```text
SIP Server
-----------
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
```
