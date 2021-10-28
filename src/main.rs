use std::io::{BufReader, Read};
use std::process::Command;

use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        let mut body = String::new();
        BufReader::new(request.as_reader()).read_to_string(&mut body).unwrap();
        open_tap(&body);

        println!("received request! method: {:?}, url: {:?}, headers: {:?}, body: {:?}",
                 request.method(),
                 request.url(),
                 request.headers(),
                 body
        );

        let response = Response::from_string("Link opened");
        request.respond(response).unwrap();
    }
}

fn open_tap(url: &String) {
    run_command(format!("firefox --new-tab {}", url).as_str())
}

fn run_command(command: &str) {
    Command::new("/usr/bin/sh")
        .args(&["-c", command])
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::null())
        .spawn().expect("Could not run the command");
}