use std::net::{SocketAddr, IpAddr};
use project1::server::Server;
use project1::client::Client;
use project1::player::Player;
use project1::game::{Move, GameResult};

#[test]
fn game_test() -> std::io::Result<()> {
    let addr: IpAddr = "127.0.0.1".parse().unwrap();
    let port = 5342;
    let sock_addr = SocketAddr::new(addr, port);
    for _ in 0..1 {


        let sock_addr_clone = sock_addr.clone();



        let mut server =
            Server::new("Test Game".to_string(), sock_addr_clone).expect("Server failed to start");
        let server_fn = move || -> std::io::Result<()> {
            server.wait_for_connect()?;
            println!("Server waiting for client ready");
            server.wait_for_enemy_ready()?;
            println!("Server sending client is ready");
            server.send_is_ready()?;

            let enemy_move = server.enemy_move()?;
            server.send_move(&Move::Paper)?;

            assert_eq!(Move::Paper.fight(&enemy_move), GameResult::Win);


            return Ok(());
        };

        let handle = std::thread::spawn(server_fn);

        let mut client = Client::new("test_client".to_string(), sock_addr).unwrap();

        println!("Client sending server it is ready");
        client.send_is_ready()?;
        println!("Client waiting for server to be ready");
        client.wait_for_enemy_ready()?;
        client.send_move(&Move::Rock)?;
        let enemy_move = client.enemy_move()?;
        assert_eq!(Move::Rock.fight(&enemy_move), GameResult::Loss);

        handle.join().unwrap()?;
    }

    Ok(())
}