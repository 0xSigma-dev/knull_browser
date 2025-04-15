use std::{io::{Read, Write}, net::TcpStream};

pub struct Fetcher;

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Trace,
    Options,
    Put,
    Delete
}

impl HttpMethod {
    fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Post => "POST",
            HttpMethod::Head => "HEAD",
        }
    }
}

impl HttpVersion {
    fn as_str(&self) -> &str {
        match self {
            HttpVersion::Http10 => "HTTP/1.O",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http20 => "HTTP/2.0"
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub address: String,
    pub version: HttpVersion,
    pub user_agent: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>
}


impl Fetcher {
    pub fn new() -> Self {
        Fetcher
    }

    pub fn fetch(&self, request: HttpRequest) -> Result<String, String> {
        println!("Received request: {:?}", &request);

        let url = request.address.trim();

        println!("url: {}", url);

        if url.is_empty() {
            return Err("URL cannot be empty".to_string())
        }

        let clean_url = url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

        println!("clean_url: {}", clean_url);

       let (host, path) = match clean_url.split_once("/") {
        Some((h, p)) => (h, format!("/{}", p)),
        None => (clean_url, String::from("/"))
       };

       let (host, port) = match host.split_once(":") {
        Some((h, p)) => (h, p.parse::<u16>().unwrap_or(80)), // Parse port or default to 80
        None => (host, 80), // Default to port 80
    };

    println!("Host: {}, Port: {}, Path: {}", host, port, &path);

       let mut stream = match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to connect: {}", e))
       };

       println!("stream: {:?}", stream);

       let method = request.method.as_str();

       println!("Method: {}", method);

       let version = request.version.as_str();

       println!("Version: {}", version);

       let mut request_line = format!("{} {} {}\r\n", method, path, version);
       println!("request_line: {}", &request_line);
       request_line.push_str(&format!("Host: {}\r\n", host));
       println!("request_line: {}", &request_line);
       request_line.push_str(&format!("User-Agent: {}\r\n", request.user_agent));
       println!("request_line: {}", &request_line);
       request_line.push_str("Connection: close\r\n");
       println!("request_line: {}", &request_line);

       for (key, value) in request.headers {
        request_line.push_str(&format!("{}: {}\r\n", key, value));
        println!("request_line: {}", &request_line);
       }

       if let Some(body) = request.body {
        request_line.push_str(&format!("Content-Length: {}\r\n", body.len()));
        println!("request_line: {}", &request_line);
        request_line.push_str("\r\n");
        println!("request_line: {}", &request_line);
        request_line.push_str(&body);
        println!("request_line: {}", &request_line);
       } else {
        request_line.push_str("\r\n");
        println!("request_line: {}", &request_line);
       };

       let request_bytes = request_line.as_bytes();
       println!("request_bytes: {:?}", &request_bytes);

       if let Err(e) = stream.write_all(request_bytes) {
        return Err(format!("Failed to send request: {}", e))
       }

       stream.flush().map_err(|e| format!("Failed to flush: {}", e))?;

       let mut response = String::new();
       let mut buffer = [0; 1024];
       println!("buffer: {:?}", buffer);
       let _ = stream.read(&mut buffer);
       println!("buffer: {:?}", buffer);
       println!("response: {}", response);
       match stream.read_to_string(&mut response) {
        Ok(_) => {
            println!("Response: {}", response);
            Ok(response)},
        Err(e) => Err(format!("Failed to read response: {}", e))
       }

    }
}


