use std::process::Command;
use tiny_http::{Method, Response, Server};

fn build(filename: &str) -> bool {
    let tmp_dir = tempdir::TempDir::new("tmp").unwrap();
    let repo = tmp_dir.path().join("cv");

    let mut c = Command::new("git");
    c.current_dir(tmp_dir.path());
    c.arg("clone");
    c.arg("https://github.com/irevoire/cv");
    if let Err(e) = c.status() {
        message(&format!("Can’t clone the repository: {:?}", e));
        return false;
    }
    let mut c = Command::new("make");
    c.current_dir(&repo);
    if let Err(e) = c.status() {
        message(&format!("Can’t build the pdf: {:?}", e));
        return false;
    }
    let mut c = Command::new("cp");
    c.current_dir(&repo);
    c.arg("main.pdf");
    c.arg(&filename);
    if let Err(e) = c.status() {
        message(&format!(
            "Can’t move the pdf to the desired destination: {:?}",
            e
        ));
        return false;
    }
    true
}

fn message(message: &str) {
    let mut notigo = Command::new("notigo");
    notigo.arg(message);
    notigo.status().unwrap();
}

fn main() {
    let filename = std::env::args()
        .skip(1) // skip the binary name
        .next()
        .expect("give me the path of your document");

    message("cv started");

    let server = Server::http("0.0.0.0:2000").unwrap();

    for request in server.incoming_requests() {
        if request.method() == &Method::Post && build(&filename) {
            message("cv updated");
            request.respond(Response::empty(200)).unwrap();
        } else {
            request.respond(Response::empty(500)).unwrap();
        }
    }
}
