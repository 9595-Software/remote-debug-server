use std::process::Command;

use tiny_http::Server;

fn main() {
    let ip = get_network_ip();
    println!("Starting remote debugger at http://[{}]:8000", ip);
    let server = Server::http(ip+":8000").unwrap();

    for mut request in server.incoming_requests() {

        // read the body
        let mut buf = Vec::new();
        request.as_reader().read_to_end(&mut buf).unwrap();
        println!("{}", String::from_utf8_lossy(&buf));
    }
}
fn get_network_ip() -> String {
    let output = Command::new("curl")
        .arg("ifconfig.me")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.to_string();
    output
}
