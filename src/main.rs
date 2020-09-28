use std::io::{stdout, Write, BufReader, stdin, BufRead};
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::exit;
use std::time::Duration;

use project1::client::Client;
use project1::game::{GameResult, Move};
use project1::player::Player;
use project1::server::Server;

fn main() -> std::io::Result<()> {
    println!("Hello! Welcome to the Rock Paper Scissors");

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.len() != 2 {
        eprintln!("Command line argument must be either\n\tserver port\nor\n\tclient ip:port");
        exit(-1);
    }

    let mut reader = BufReader::new(stdin());
    print!("Enter your name: ");
    stdout().flush()?;

    let mut name = String::new();
    reader.read_line(&mut name)?;

    if args[0].to_lowercase() == "server" {
        server(&args, name.trim_end())?;
    } else if args[0].to_lowercase() == "client" {
        client(&args, name.trim_end())?;
    } else {
        eprintln!("Either client or server must be the first argument");
        exit(-1);
    };
    Ok(())
}

/// Runs the client
///
/// Client will always have priority over the stream, where it sends info first then the server responds
fn client(args: &[String], name: &str) -> std::io::Result<()> {
    let addrs = args[1]
        .to_socket_addrs()
        .expect("Second argument is a not valid socket address");
    let mut found_client = None;
    for addr in addrs {
        let client = Client::new(name.to_string(), addr);
        if let Ok(client) = client {
            found_client = Some(client);
            break;
        }
    }

    match found_client {
        None => {
            eprintln!("No server was found");
            exit(-1);
        }
        Some(mut client) => {
            let my_move = client.my_move();

            client.send_is_ready()?; // client sends to the server it's ready
            client.wait_for_enemy_ready()?; // client waits for the server to send back that it's ready

            client.send_move(&my_move)?; // client sends its move
            let enemy_move = client.enemy_move()?; // client get the server's move
            end_game(&client, &my_move, &enemy_move);
            Ok(())
        }
    }
}

/// Runs the server
fn server(args: &[String], name: &str) -> std::io::Result<()> {
    let port: u16 = args[1]
        .parse()
        .expect("Second argument is not a valid port");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let mut server = Server::new(name.to_string(), addr)?;
    server.wait_for_connect()?;

    let my_move = server.my_move();
    server.wait_for_enemy_ready()?; // Server waits for the client to let it know it's ready
    server.send_is_ready()?; // Server tells client it too is ready



    let enemy_move = server.enemy_move()?; // Gets the enemy move
    server.send_move(&my_move)?; // server sends its move to the client

    end_game(&server, &my_move, &enemy_move);
    Ok(())
}

/// Mutually shared behavior between the client and the server that ends the game
fn end_game(player: &dyn Player, my_move: &Move, enemy_move: &Move) {
    for i in (1..=3).rev() {
        if i == 1 {
            println!("1")
        } else {
            print!("{}, ", i);
            stdout().flush().unwrap();
        }
        std::thread::sleep(Duration::from_secs_f32(0.5));
    }
    println!(
        "{} (you) played: {}    {} played: {}",
        player.my_name(),
        my_move,
        player.enemy_name().unwrap(),
        enemy_move
    );
    let result = my_move.fight(&enemy_move);
    match result {
        GameResult::Win => println!("You won!"),
        GameResult::Loss => println!("You lost!"),
        GameResult::Tie => println!("You tied!"),
    }
}
