use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::path::PathBuf;

pub fn serve(addr: SocketAddr, content: String) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let resp = format!(
        "HTTP/1.0 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    let resp = resp.as_bytes();
    loop {
        let (mut stream, _addr) = listener.accept()?;
        if let Err(e) = stream.write_all(resp) {
            eprintln!("simple_http: write error: {:?}", e);
        }
    }
}

pub fn serve_many(addr: SocketAddr, files: HashMap<String, PathBuf>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    loop {
        let (mut stream, _addr) = listener.accept()?;
        let mut rdr = std::io::BufReader::new(stream.try_clone()?);
        let mut path = String::new();
        rdr.read_line(&mut path)?;
        let prefix = "GET ";
        let suffix = " HTTP/1.1\r\n";
        if !path.starts_with(prefix) {
            if let Err(e) = stream.write_all(b"HTTP/1.0 405 Method Not Allowed\r\n\r\n") {
                eprintln!("simple_http: write error: {:?}", e);
            }
            continue;
        }
        if !path.ends_with(suffix) {
            if let Err(e) = stream.write_all(b"HTTP/1.0 400 Bad Request\r\n\r\n") {
                eprintln!("simple_http: write error: {:?}", e);
            }
            continue;
        }
        let path = &path[prefix.len()..(path.len() - suffix.len())];
        if let Some(path) = files.get(path) {
            let data = std::fs::read(path)?;

            let resp = format!(
        "HTTP/1.0 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
        data.len(),
    );
            let mut resp = resp.into_bytes();
            resp.extend(data);
            if let Err(e) = stream.write_all(&resp) {
                eprintln!("simple_http: write error: {:?}", e);
            }
        } else {
            if let Err(e) = stream.write_all(b"HTTP/1.0 404 Not Found\r\n\r\n") {
                eprintln!("simple_http: write error: {:?}", e);
            }
        }
    }
}
