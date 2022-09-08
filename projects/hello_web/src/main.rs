use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
    sync::{Arc, Mutex},
    ops::Range
};
use hello_web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let request_id = Arc::new(Mutex::new(0));
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let request_id = Arc::clone(&request_id);

        pool.execute(move || {
            let id = get_request_id(request_id);
            handle_connection(stream, id);
        });
    }

    println!("Shutting down!");
}

fn get_request_id(request_id_mutex: Arc<Mutex<usize>>) -> usize {
    let mut request_id = request_id_mutex.lock().unwrap();
    let id = *request_id;
    *request_id += 1;
    id
}

fn handle_connection(mut stream: TcpStream, request_id: usize) {
    println!("Processing request {request_id}...");

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(0));
            let ns: Range<usize> = 1..500000000;
            ns.for_each(|n| { let _ = n * n; });
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("... request {request_id} done!");
}
