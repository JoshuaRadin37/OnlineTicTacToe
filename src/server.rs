use std::env::set_current_dir;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};

use crate::game::Move;
use crate::player::Player;

pub struct Server {
    server_name: String,
    listener: TcpListener,
    client: Option<TcpStream>
}

impl Server {

    pub fn new<A : ToSocketAddrs>(name: String, addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server {
            server_name: name,
            listener,
            client: None
        })
    }

    pub fn wait_for_connect(&mut self) -> std::io::Result<()> {
        let mut connection = self.listener.accept()?;
        println!("Client Connected from Address: {:?}", connection.1);
        let mut stream = connection.0;
        {
            let mut buffered_reader = BufReader::new(&stream);
            let mut confirm_message = String::new();
            buffered_reader.read_line(&mut confirm_message)?;
            if confirm_message != "RPS Client Game Connect\n" {
                panic!("Connection not the client for this game");
            }
            let mut writer = BufWriter::new(&stream);
            writeln!(writer, "RPS Server Game Connection Confirmed")?;
            writeln!(writer, "{}", self.server_name)?;
        }
        self.client = Some(stream);
        Ok(())
    }
}

impl Player for Server {
    fn send_move(&mut self, mov: &Move) -> Result<()> {
        let mut writer = BufWriter::new(self.client.as_ref().unwrap());
        writeln!(writer, "{}", mov.to_string())
    }

    fn enemy_move(&self) -> Result<Move> {
        let mut reader = BufReader::new(self.client.as_ref().unwrap());
        let mut enemy_move: String = String::new();
        reader.read_line(&mut enemy_move)?;
        Ok(enemy_move.into())
    }
}


