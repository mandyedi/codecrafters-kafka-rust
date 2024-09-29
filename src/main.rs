#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener, mem};
use bytes::{Buf, BufMut};

struct RequestHeader {
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
}

struct ResponseHeader {
    message_length: i32,
    correlation_id: i32,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut length: [u8; 4] = [0, 0, 0, 0];
                stream.read_exact(&mut length).unwrap();
                let length = i32::from_be_bytes(length) as usize;

                let mut request = vec![0; length];
                stream.read_exact(&mut request).unwrap();
                
                let mut request = request.as_slice();
                let request_header = RequestHeader {
                    request_api_key: request.get_i16(),
                    request_api_version: request.get_i16(),
                    correlation_id: request.get_i32()
                };

                let response_header = ResponseHeader {
                    message_length: 0,
                    correlation_id: request_header.correlation_id,
                };

                let mut response = Vec::with_capacity(10);
                response.put_i32(response_header.message_length);
                response.put_i32(response_header.correlation_id);

                if 0 > request_header.request_api_version || 4 < request_header.request_api_version {
                    response.put_i16(35);
                }

                stream.write_all(&response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
