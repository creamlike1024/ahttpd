use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

#[derive(Clone, Copy,Debug)]  // 为了能够复制，所以需要添加 Clone 和 Copy 特性
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    // 将状态码转换为字符串
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
