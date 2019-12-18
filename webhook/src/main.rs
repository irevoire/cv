use std::process::Command;
use tiny_http::{Method, Response, Server};

fn build(filename: &str) {
    let tmp_dir = tempdir::TempDir::new("tmp").unwrap();
    let repo = tmp_dir.path().join("cv");

    let mut c = Command::new("git");
    c.current_dir(tmp_dir.path());
    c.arg("clone");
    c.arg("https://github.com/irevoire/cv");
    c.status();
    let mut c = Command::new("make");
    c.current_dir(&repo);
    c.status();
    let mut c = Command::new("cp");
    c.current_dir(&repo);
    c.arg("main.pdf");
    c.arg(&filename);
    c.status();
}

fn message(message: &str) {
    let mut notigo = Command::new("notigo");
    notigo.arg(message);
    notigo.status();
}

fn main() {
    let filename = std::env::args()
        .skip(1) // skip the binary name
        .next()
        .expect("give me the path of your document");

    message("cv started");

    let server = Server::http("0.0.0.0:2000").unwrap();

    for request in server.incoming_requests() {
        if request.method() == &Method::Post {
            build(&filename);
            message("cv updated");
            request.respond(Response::empty(200));
        } else {
            request.respond(Response::empty(500));
        }
    }
}
