use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request,Response,StatusCode, response, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        match TcpListener::bind(&self.addr){
            Ok(listener) => {
                println!("Listening on {}", self.addr);
                loop {
                    // 无限循环，等待连接
                    match listener.accept() {
                        Ok((mut stream, addr)) => {
                            println!("Connection established with {}", addr);
                            let mut buffer = [0; 1024];
                            // read 适用于可变引用
                            match stream.read(&mut buffer){
                                Ok(_size) => {
                                    // from_utf8_lossy 确保转换为字符串时不会出错
                                    // println!("Request: {}", String::from_utf8_lossy(&buffer));
                                    let response = match Request::try_from(&buffer[..]){
                                        Ok(request) => handler.handle_request(&request),
                                        Err(e) => handler.handle_bad_request(&e),
                                    };
                                    if let Err(e) = response.send(&mut stream) {
                                        println!("Failed to send response: {}", e);
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to read from connection: {}", e);
                                }
                            }
                        }
                        Err(e) => println!("Connection failed: {}", e),
                    }
                }
            }
            Err(e) => println!("Failed to bind: {}", e),
        }
    }
}