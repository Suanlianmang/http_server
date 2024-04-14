

#[derive(Debug)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}


impl HttpVersion {
    pub fn from_string(value: &str) -> Option<HttpVersion> {
        let value = value.trim();
        let value = value.to_uppercase();

        if value == "HTTP/1.1"{
            return Some(Self::Http1_1);
        }
        if value == "HTTP/1.0"{
            return Some(Self::Http1_0);
        }
        if value == "HTTP/2.0"{
            return Some(Self::Http2_0);
        }
        if value == "HTTP/3.0"{
            return Some(Self::Http3_0);
        }
        None
    }
}
