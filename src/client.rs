use std::io::Result;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::game::Move;
use crate::player::Player;
use std::convert::TryInto;
use std::process::exit;

/// Client structs can only be created if a successful connection to a server is made. The name
/// of the client and the server are both stored, along with the TcpStream to the server.
/// # Example
/// ```
/// use project1::client::Client;
/// use project1::player::Player;
/// use project1::game::Move;
/// let mut client = Client::new("Client Name".to_string(), "localhost:1234").unwrap();
/// client.send_move(&Move::Rock);
/// let enemy_move = client.enemy_move();
/// ```
pub struct Client {
    name: String,
    enemy_name: String,
    enemy_stream: TcpStream,
}

impl Client {
    /// Creates a new client struct if it can successfully connect to a RSP server. If an io error occurs
    /// , an error is returned.
    pub fn new<A: ToSocketAddrs>(name: String, address: A) -> std::io::Result<Client> {
        let mut stream = TcpStream::connect_timeout(
            &address.to_socket_addrs()?.next().unwrap(),
            Duration::from_secs(15),
        )?;
        writeln!(stream, "RPS Client Game Connect")?; // Sends a message to the server as soon as it attaches

        let mut buffered_reader = BufReader::new(&stream);
        let mut confirm_message = String::new();
        buffered_reader.read_line(&mut confirm_message)?;
        if confirm_message != "RPS Server Game Connection Confirmed\n" { // waits for the server's confirmation
            panic!("Connection not the server for this game");
        }

        confirm_message.clear();
        buffered_reader.read_line(&mut confirm_message)?; // Gets the name of the server player
        let server_name = confirm_message.trim_end();
        println!("Joined {}'s Game", server_name);

        {
            let mut writer = BufWriter::new(&stream);
            writeln!(writer, "{}", name)?; // Sends the client player's name
        }

        Ok(Client {
            name,
            enemy_name: server_name.to_string(),
            enemy_stream: stream,
        })
    }
}

impl Player for Client {
    fn send_move(&mut self, mov: &Move) -> Result<()> {
        let mut writer = BufWriter::new(&self.enemy_stream);
        writeln!(writer, "{}", mov.to_string())
    }

    fn enemy_move(&self) -> Result<Move> {
        let mut reader = BufReader::new(&self.enemy_stream);
        let mut enemy_move: String = String::new();
        reader.read_line(&mut enemy_move)?;
        Ok(enemy_move.trim_end().to_string().try_into().expect(format!("{} isn't a valid move", enemy_move).as_str()))
    }

    fn send_is_ready(&self) -> Result<()> {
        let mut writer = BufWriter::new(&self.enemy_stream);
        writeln!(writer, "CLIENT READY")
    }

    fn wait_for_enemy_ready(&self) -> Result<()> {
        let mut reader = BufReader::new(&self.enemy_stream);
        let mut enemy_move: String = String::new();
        let read = reader.read_line(&mut enemy_move)?;
        if enemy_move == "SERVER READY\n" {
            Ok(())
        } else if read == 0 {
            eprintln!("{} disconnected", self.enemy_name);
            exit(-1);
        } else {
            eprintln!("Invalid message received while waiting: {}", enemy_move);
            exit(-1);
        }
    }


    fn my_name(&self) -> &str {
        self.name.as_ref()
    }

    fn enemy_name(&self) -> Option<&str> {
        Option::Some(self.enemy_name.as_ref())
    }
}
