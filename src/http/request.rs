use super::core::HttpVersion;

#[derive(Debug)]
pub enum HttpMethod {
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
impl HttpMethod {
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

    fn from_string(value: &str) -> Option<HttpMethod> {
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
pub struct Request {
    pub http_request: Vec<String>,
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub host: String,
    pub path: String,
    pub connection: String,
    pub user_agent: String,
    pub referer: String,
    pub cache_control: String,
    pub accept: Vec<String>,
    pub accept_encoding: Vec<String>,
    pub accept_language: Vec<String>,
}

impl Request {
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
        let method = HttpMethod::from_string(request_line.first().unwrap()).unwrap();
        let path = request_line[1].to_string();


        let mut request = Request {
            method,
            http_request: Vec::new(),
            path,
            http_version: HttpVersion::from_string(request_line.last().unwrap()).unwrap(),
            host: String::new(),
            connection: String::new(),
            user_agent: String::new(),
            referer: String::new(),
            cache_control: String::new(),
            accept: Vec::new(),
            accept_encoding: Vec::new(),
            accept_language: Vec::new(),
        };

       for request_field in http_request.iter().skip(1){
            if request_field.is_empty() {
                continue
            }
            let request_field: [String; 2] = match request_field.trim().split_once(":") {
                Some((key, value)) => [key.trim().to_lowercase(), value.trim().to_string()],
                None => panic!("Cannot process request field"),         
            };
            println!("{:?}", request_field);

            match request_field.first().unwrap().as_str() {
                "host" => {
                    request.host = request_field[1].clone();
                },
                "connection" => {
                    request.connection = request_field[1].clone();
                },
                "referer" => {
                    request.referer = request_field[1].clone();
                },
                "user-agent" => {
                    request.user_agent = request_field[1].clone();
                },
                "cache-control" => {
                    request.cache_control = request_field[1].clone();
                },
                 
                _ => (),
            }
        }
        // request.http_request = http_request;

        request
    }
}
