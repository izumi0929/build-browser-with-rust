pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
}

extern crate alloc;
use alloc::string::String;
use sava_core::error::Error;
use sava_core::http::HttpResponse;
use noli::net::SocketAddr;
use alloc::vec::Vec;

impl HttpClient {
    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP Addresses: {:#?}",
                    e
                )))
            }
        };
        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP Addresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to connect to TCP stream".to_string(),
                    e
                )))
            }
        };

        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");
        // ヘッダの追加
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str("\n");
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push_str("\n");

        let _bytes_written =  match stream.write(request.as_bytes()) {
            Ok(bytes_written) => bytes_written,
            Err(_) => {
                return Err(Error::Network(format!(
                    "Failed to send a request to TCP stream".to_string(),
                )))
            }
        };

        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(format!(
                        "Failed to receive a request from TCP stream".to_string(),
                    )))
                }
            };
            if bytes_read == 0 {
                break;
            };
            received.extend_from_slice(&buf[..bytes_read]);
        }

        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(_) => Err(Error::Network(format!("Invalid received response: {}", e))),
        }

    }
}