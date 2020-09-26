use std::convert::TryInto;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use crate::game::Move;
use crate::player::Player;

/// The server struct. The server must be created first before a client can run. The server can only
/// hook into one client. Once the client has connected, the game runs to completion. Once a server
/// is dropped, the TcpListener is also dropped and the port is closed.
pub struct Server {
    server_name: String,
    listener: TcpListener,
    client: Option<TcpStream>,
    enemy_name: Option<String>,
}

impl Server {
    /// Creates a new server, with the player name and at an address. The address should be a local address,
    /// either being local host or `0.0.0.0`
    pub fn new<A: ToSocketAddrs>(name: String, addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server {
            server_name: name,
            listener,
            client: None,
            enemy_name: None,
        })
    }

    /// Tells the server to wait for a client to connect. The server will check to see if the process that
    /// connects to the server is an RCP client. The process after a process connects to the server goes as follows:
    ///
    /// 1. Client sends a specific message to this server establishing that is an RCP client
    /// 2. The server then sends back a message to the client letting it know that this is an RCP server
    /// 3. The server then sends the name of the server player
    /// 4. The client then sends the name of the client player
    ///
    /// If any of the above steps fail, the function fails and returns an error.
    pub fn wait_for_connect(&mut self) -> std::io::Result<()> {
        let connection = self.listener.accept()?;

        let stream = connection.0;
        {
            let mut buffered_reader = BufReader::new(&stream);
            let mut confirm_message = String::new();
            buffered_reader.read_line(&mut confirm_message)?;
            if confirm_message != "RPS Client Game Connect\n" {
                panic!("Connection not the client for this game");
            }

            {
                let mut writer = BufWriter::new(&stream);
                writeln!(writer, "RPS Server Game Connection Confirmed")?;
                writeln!(writer, "{}", self.server_name)?;
            }

            let mut enemy_name = String::new();
            buffered_reader.read_line(&mut enemy_name)?;
            let enemy = enemy_name.trim_end().to_string();
            println!("{} has joined the game", enemy);
            self.enemy_name = Some(enemy);
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
        Ok(enemy_move.trim_end().to_string().try_into().unwrap())
    }

    fn my_name(&self) -> &str {
        self.server_name.as_ref()
    }

    fn enemy_name(&self) -> Option<&str> {
        self.enemy_name.as_deref()
    }
}
