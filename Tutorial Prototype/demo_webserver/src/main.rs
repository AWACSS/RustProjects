


use::std::net::TcpListener;
use std::{net::TcpStream, io::{Read, Write, BufReader, BufRead}, fs};




fn main() {

    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){

        let stream = stream.unwrap();
        println!("connection established!");
        handle_connection(stream);
    }

}


fn handle_connection( mut stream: TcpStream){

    let mut buffer = [0;512];
    stream.read( &mut buffer).unwrap();


    // 请求
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body


    let buf_reader = BufReader::new(&mut stream);
    //请求
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();


    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();



    //响应 
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
    let response =
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    

    println!("request: {}",String::from_utf8_lossy( &buffer[..]));





}










