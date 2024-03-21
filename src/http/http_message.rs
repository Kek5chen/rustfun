use std::{collections::HashMap};

#[allow(dead_code)]
pub struct HTTPMessage {
    pub request_type: String,
    pub path: String,
    pub protocol: String,
    pub header: HashMap<String, String>,
    pub data: String,
}

#[allow(dead_code)]
impl HTTPMessage {
    pub fn new() -> Self {
        HTTPMessage {
            request_type: String::new(),
            path: String::from("/"),
            protocol: String::from(""),
            header: HashMap::new(),
            data: String::new(),
        }
    }

    pub fn parse_request(data: &str) -> Result<Self, String> {
        let (info, header, body) = Self::split_request(data);

        let mut words = info.split_whitespace();
        let request_type = words.next().unwrap_or("").to_string();
        let path = words.next().unwrap_or("/").to_string();
        let protocol = words.next().unwrap_or("HTTP/1.1").to_string();

        Ok(HTTPMessage {
            request_type,
            path,
            protocol,
            header: Self::parse_header(header),
            data: body.to_string(),
        })
    }

    // returns tuple of info, header, body as strings
    fn split_request(request: &str) -> (&str, &str, &str) {
        let mut lines = request.lines();
        let info = lines.next().unwrap_or("");
        let header_end = match info.find("\r\n\r\n") {
            Some(header_end) => header_end,
            None => return (info, &request[info.len()..], ""),
        };
        let header = &request[info.len()..header_end];
        let body = &request[header_end..];

        (info, header, body)
    }

    fn parse_header(header: &str) -> HashMap<String, String> {
        let lines = header.lines();
        let mut header_map: HashMap<String, String> = HashMap::new();

        for line in lines {
            match line.find(": ") {
                Some(split) => { header_map.insert(
                        (&line[..split]).to_string(),
                        (&line[split + 2..]).to_string()); },
                None => continue,
            }
        }

        header_map
    }

    pub fn make_response(&self) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<h1>literally mowserver</h1>".to_string()
    }

    pub fn get(&self, field_name: &str) -> Option<&String> {
        self.header.get(field_name)
    }

    pub fn add(&mut self, field_name: &str, value: &str) {
        self.header.insert(field_name.to_string(), value.to_string());
    }
}

