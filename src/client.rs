use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Write, BufReader, BufRead};

pub struct Client {
    stream: TcpStream
}

impl Client {

    pub fn new<A : ToSocketAddrs>(address: A) -> std::io::Result<Client> {
        let mut stream = TcpStream::connect(address)?;
        write!(stream, "RPS Client Game Connect")?;

        let mut buffered_reader = BufReader::new(&stream);
        let mut confirm_message = String::new();
        buffered_reader.read_line(&mut confirm_message)?;
        if confirm_message != "RPS Server Game Connection Confirmed" {
            panic!("Connection not the server for this game");
        }
        buffered_reader.read_line(&mut confirm_message)?;
        println!("Joined Game: {}", confirm_message);

        Ok(Client {
            stream
        })
    }
}

