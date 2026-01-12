use learn_http::dual_roles::connect;
use learn_http::fd::run_main;
use learn_http::mcs;
use learn_http::mesg_bndry;
use learn_http::no_partial_reads;
use learn_http::server::Server;

fn main() {
    // let server = Server::new("127.0.0.1:8080");

    // server.connect();

    // connect();
    // fd::run_main();
    // mesg_bndry::run_main();
    // no_partial_reads::connect();
    if let Err(e) = mcs::MCS::connect("127.0.0.1:8080") {
        eprintln!("Error: {}", e);
    }
}
