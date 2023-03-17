use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use super::status_code::StatusCode;

pub struct Response {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,   // body 可以为空，所以使用 Option 类型
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            headers: HashMap::new(),
            body,
        }
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        // 将响应写入到 f 中
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body)
        
    }
}