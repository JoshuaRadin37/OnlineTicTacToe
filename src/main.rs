use project1::server::Server;
use std::process::exit;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use project1::client::Client;
use std::io::Error;
use project1::player::Player;
use project1::game::{GameResult, Move};

fn main() {
    println!("Hello! Welcome to the Rock Paper Scissors");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Command line argument must be either\n\tserver port\nor\n\tclient ip:port");
        exit(-1);
    }

    let name = &args[2];

    if args[0].to_lowercase() == "server" {
        server(&args, name);
    } else if args[0].to_lowercase() == "client" {
        client(&args);
    } else {
        eprintln!("Either client or server must be the first argument");
        exit(-1);
    };

}

fn client(args: &Vec<String>) {
    let addrs = args[1].to_socket_addrs().expect("Second argument is a not valid socket address");
    let mut found_client = None;
    for addr in addrs {
        let client = Client::new(args[2].clone(), addr);
        match client {
            Ok(client) => {
                found_client = Some(client);
                break;
            },
            Err(_) => {}
        }
    }

    match found_client {
        None => {
            eprintln!("No server was found");
            exit(-1);
        }
        Some(mut client) => {
            let my_move = client.my_move();
            client.send_move(&my_move).expect("Failed to send move");
            let enemy_move = client.enemy_move().expect("Failed to receive move");
            end_game(&my_move, &enemy_move)
        }
    }

}

fn server(args: &Vec<String>, name: &String){
    let port: u16 = args[1].parse().expect("Second argument is not a valid port");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Tell Opponent to connect to this address: {}", addr);
    let mut server = Server::new(name.clone(), addr).expect(format!("Could not open socket on port {}", port).as_ref());
    server.wait_for_connect().expect("No client connected");

    let my_move = server.my_move();
    server.send_move(&my_move).expect("Failed to send move");
    let enemy_move = server.enemy_move().expect("Failed to receive move");

    end_game(&my_move, &enemy_move)
}

fn end_game(my_move: &Move, enemy_move: &Move) {
    println!("You played: {}    Enemy player: {}", my_move, enemy_move);
    let result = my_move.fight(&enemy_move);
    match result {
        GameResult::Win => println!("You won!"),
        GameResult::Loss => println!("You lost!"),
        GameResult::Tie => println!("You tied!")
    }
}



