use super::server::Handler;
use super::http::{Response,Request,StatusCode,Method};
use std::fs;

pub struct HttpHandler {
    website_path: String,
}

impl HttpHandler {
    pub fn new(website_path: String) -> Self {
        Self { website_path }
    }
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        // 检查路径是否合法
        let path = format!("{}/{}", self.website_path, file_path);
        if !path.starts_with(&self.website_path) {
            println!("Directory Traversal Attack Attempted: {}", file_path);
            return None;
        }
        match fs::read_to_string(path) {
            Ok(contents) => Some(contents),
            Err(_) => None,
        }
    }
}

impl Handler for HttpHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                // 将未匹配的路径作为 path 变量传递给 read_file 函数
                path => match self.read_file(path){
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}