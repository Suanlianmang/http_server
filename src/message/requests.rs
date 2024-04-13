use super::http::HttpVersion;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT
}
impl Method {
    const fn to_str(&self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
            Self::DELETE => "DELETE",
            Self::OPTIONS => "OPTIONS",
            Self::HEAD => "HEAD",
            Self::TRACE => "TRACE",
            Self::CONNECT => "CONNECT",
        }
    }

    fn from_string(value: &str) -> Option<Method> {
        let value = value.trim();
        let value = value.to_uppercase();
        if value == "GET"{
            return Some(Self::GET);
        }
        if value == "POST"{
            return Some(Self::POST);
        }
        if value == "PUT"{
            return Some(Self::PUT);
        }
        if value == "PATCH"{
            return Some(Self::PATCH);
        }
        if value == "DELETE"{
            return Some(Self::DELETE);
        }
        if value == "OPTIONS"{
            return Some(Self::OPTIONS);
        }
        if value == "HEAD"{
            return Some(Self::HEAD);
        }
        if value == "TRACE"{
            return Some(Self::TRACE);
        }
        if value == "CONNECT"{
            return Some(Self::CONNECT);
        }

       None 
    }
}


#[derive(Debug)]
pub struct Requests {
    pub http_request: Vec<String>,
    pub method: Method,
    pub http_version: HttpVersion,
    pub path: String,
}

impl Requests {
    pub fn new(http_request: Vec<String>) -> Self {
        if http_request.len() < 2 {
            panic!("Requerst is too short!");
        }

        let request_line: Vec<_> = http_request
            .first()
            .unwrap()
            .split(' ')
            .into_iter()
            .collect();

        if request_line.len() != 3 {
            panic!("Request line is wrong");
        } 
        let method = Method::from_string(request_line.first().unwrap()).unwrap();
        let path = request_line[1].to_string();

        Requests {
            method,
            http_request,
            path,
            http_version: HttpVersion::Http1_1,
        }
    }
}
