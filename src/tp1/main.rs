use std::{
    io::{prelude::*, BufReader},
    time::{Instant},
    net::{TcpListener, TcpStream},
};

fn main() {

    // Listens to TCP connections and binds it to port 7878
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    // "listener.incoming()" returns iterator of connection instances
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established successfully!!!");
        handle_connection(stream);
    }
}

// Method works with large numbers. Very important to use u128!

fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Extract the path into variable
    let path = request_line.split_whitespace().nth(1).unwrap();

    // Split the path
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    // Get the method requested
    let method = parts.get(0).unwrap_or(&"");
    //Get the argument given to the method
    let argument = parts.get(1).unwrap_or(&"");

    // Handle the method based on string value
    match *method {
        "pi" => {
            // Parses value and if successfully continues inside if
            let parsed_result = argument.parse::<u128>();
            if let Ok(i) = parsed_result {
                // now and elapsed variables are used to calculate time passed
                let now = Instant::now();
                let result = calculate_pi(i);
                let elapsed = now.elapsed().as_secs_f64();

                println!("Pi to {i} terms of Leibniz is: {result}");
                let response = format!("HTTP/1.1 200 OK\r\n\r\nPi to {i} terms of Liebniz is: {result} calculated in {elapsed} seconds");
                stream.write_all(response.as_bytes()).unwrap();
            } else {
                let response = format!("HTTP/1.1 400 Bad Request\r\n\r\nInvalid argument: {argument}");
                println!("{response}");
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
        _ => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\nMethod not found";
            println!("{response}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

// Calculates pi with Liebniz series to the "i-th" term
fn calculate_pi(i: u128) -> f64 {
    let mut pi = 0.0;
    for k in 0..i {
        let term = ((-1.0_f64).powi(k as i32)) / (2 * k + 1) as f64;
        pi += term;
    }
    pi * 4.0
}
