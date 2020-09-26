use std::io::{BufRead, BufReader, Write, BufWriter};
use std::io::Result;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::game::Move;
use crate::player::Player;
use std::convert::TryInto;

pub struct Client {
    name: String,
    enemy_name: String,
    stream: TcpStream
}

impl Client {

    pub fn new<A : ToSocketAddrs>(name: String, address: A) -> std::io::Result<Client> {
        let mut stream = TcpStream::connect_timeout(&address.to_socket_addrs()?.next().unwrap(), Duration::from_secs(15))?;
        writeln!(stream, "RPS Client Game Connect")?;

        let mut buffered_reader = BufReader::new(&stream);
        let mut confirm_message = String::new();
        buffered_reader.read_line(&mut confirm_message)?;
        if confirm_message != "RPS Server Game Connection Confirmed\n" {
            panic!("Connection not the server for this game");
        }


        confirm_message.clear();
        buffered_reader.read_line(&mut confirm_message)?;
        let server_name = confirm_message.trim_end();
        println!("Joined {}'s Game", server_name);

        {
            let mut writer = BufWriter::new(&stream);
            writeln!(writer, "{}", name)?;
        }

        Ok(Client {
            name,
            enemy_name: server_name.to_string(),
            stream,
        })
    }
}

impl Player for Client {

    fn send_move(&mut self, mov: &Move) -> Result<()> {
        let mut writer = BufWriter::new(&self.stream);
        writeln!(writer, "{}", mov.to_string())
    }

    fn enemy_move(&self) -> Result<Move> {
        let mut reader = BufReader::new(&self.stream);
        let mut enemy_move: String = String::new();
        reader.read_line(&mut enemy_move)?;
        Ok(enemy_move.trim_end().to_string().try_into().unwrap())
    }

    fn my_name(&self) -> &str {
        self.name.as_ref()
    }

    fn enemy_name(&self) -> Option<&str> {
        Option::Some(self.enemy_name.as_ref())
    }
}

