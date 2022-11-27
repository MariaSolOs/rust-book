use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request = buf_reader.lines().next().unwrap().unwrap();

    let (content_path, status) = if request == "GET / HTTP/1.1" {
        ("hello.html", "200 OK")
    } else {
        ("404.html", "404 NOT FOUND")
    };

    let content = fs::read_to_string(content_path).unwrap();

    let response = format!(
        "HTTP/1.1{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        if let Ok(stream) = stream {
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
}
