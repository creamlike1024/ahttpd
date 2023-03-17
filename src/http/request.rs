#![allow(unused)]
use std::collections::HashMap;
use crate::http::request;

use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult,Debug};
use std::str::{self, Utf8Error};
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buf> { 
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    // 示例 HTTP 请求：
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request=str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; // 与上面的 match 语句等价
        let (method,request) = Request::get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = Request::get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = Request::get_next_word(request).ok_or(ParseError::InvalidProtocol)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        //match path.find('?'){
        //    Some(i) => {
        //        // find() 返回字符串中第一个匹配的索引，如果没有匹配的则返回 None
        //        // 找到了 ?，则将 query_string 提取出来，i+1 是为了去除 ?
        //        query_string = Some(path[i+1..].to_string());
        //        // 提取 path
        //        path = &path[..i];
        //    },
        //    None => {}
        //}
        // 使用 if let 语句来简化 match 语句
        if let Some(i) = path.find('?') {
             query_string = Some(QueryString::from(&path[i+1..]));
             path = &path[..i];
        }
        // 返回 Request
        Ok(Self {
            path: path,
            query_string,
            method,
            headers: HashMap::new(),
            body: None,
        })
    }
}

impl<'buf> Request<'buf> {
    fn get_next_word(request: &str) -> Option<(&str,&str)> {
        for (i,c) in request.chars().enumerate() {
            if c == ' ' || c== '\r' {
                // 返回的第二个字符串去掉空格或者换行符
                return Some((&request[..i],&request[i+1..]));
            }
        }
        None
    }
    pub fn path(&self) -> &str {
        // 返回 path
        &self.path
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        // 返回 query_string
        self.query_string.as_ref()
    }
    pub fn method(&self) -> &Method {
        // 返回 method
        &self.method
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        // 返回 headers
        &self.headers
    }
    pub fn body(&self) -> Option<&str> {
        // 返回 body，由于 body 是 Option<String> 类型，所以需要 as_deref() 转换为 Option<&str>
        self.body.as_deref()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidEncoding
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}

