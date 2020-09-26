use project1::client::Client;
use project1::server::Server;
use std::net::{IpAddr, SocketAddr};

#[test]
fn local_test() {
    let addr: IpAddr = "127.0.0.1".parse().unwrap();
    let port = 5342;
    let sock_addr = SocketAddr::new(addr, port);
    let sock_addr_clone = sock_addr.clone();
    let mut server =
        Server::new("Test Game".to_string(), sock_addr_clone).expect("Server failed to start");
    let server_fn = move || -> std::io::Result<()> {
        server.wait_for_connect()?;
        return Ok(());
    };

    let handle = std::thread::spawn(server_fn);

    let _client = Client::new("test_client".to_string(), sock_addr).unwrap();

    handle.join().unwrap().unwrap();
}
