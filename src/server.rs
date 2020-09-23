use std::net::{SocketAddrV4, TcpListener, ToSocketAddrs, TcpStream};
use crate::player::Player;
use crate::game::Move;
use std::env::set_current_dir;
use std::io::{BufReader, BufRead, BufWriter, Write};

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
            if confirm_message != "RPS Client Game Connect" {
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
    fn my_move(&mut self) -> Move {
        unimplemented!()
    }

    fn enemy_move(&self, enemy: &mut Self) -> Move {
        unimplemented!()
    }
}


