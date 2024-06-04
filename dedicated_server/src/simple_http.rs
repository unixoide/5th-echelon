use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::path::PathBuf;

use slog::debug;
use slog::error;

pub fn serve(logger: &slog::Logger, addr: SocketAddr, content: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let resp = format!(
        "HTTP/1.0 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    let resp = resp.as_bytes();
    loop {
        let (mut stream, _addr) = listener.accept()?;
        let mut rdr = std::io::BufReader::new(stream.try_clone()?);
        let mut path = String::new();
        if let Err(e) = rdr.read_line(&mut path) {
            error!(logger, "read error: {:?}", e);
            continue;
        }
        debug!(logger, "Request: {}", path);
        if let Err(e) = stream.write_all(resp) {
            error!(logger, "write error: {:?}", e);
        }
    }
}

pub fn serve_many(
    logger: &slog::Logger,
    addr: SocketAddr,
    files: &HashMap<String, PathBuf>,
) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    loop {
        let (mut stream, _addr) = listener.accept()?;
        let mut rdr = std::io::BufReader::new(stream.try_clone()?);
        let mut path = String::new();
        if let Err(e) = rdr.read_line(&mut path) {
            error!(logger, "simple_http: read error: {:?}", e);
            continue;
        }
        debug!(logger, "Request: {}", path);
        let prefix = "GET ";
        let suffix = " HTTP/1.1\r\n";
        if !path.starts_with(prefix) {
            debug!(logger, "Status 405");
            if let Err(e) = stream.write_all(b"HTTP/1.0 405 Method Not Allowed\r\n\r\n") {
                error!(logger, "simple_http: write error: {:?}", e);
            }
            continue;
        }
        if !path.ends_with(suffix) {
            debug!(logger, "Status 400");
            if let Err(e) = stream.write_all(b"HTTP/1.0 400 Bad Request\r\n\r\n") {
                error!(logger, "simple_http: write error: {:?}", e);
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
            debug!(logger, "Status 200");
            let mut resp = resp.into_bytes();
            resp.extend(data);
            if let Err(e) = stream.write_all(&resp) {
                error!(logger, "simple_http: write error: {:?}", e);
                continue;
            }
        } else if let Err(e) = stream.write_all(b"HTTP/1.0 404 Not Found\r\n\r\n") {
            error!(logger, "simple_http: write error: {:?}", e);
            continue;
        }
        debug!(logger, "Status 404");
    }
}
