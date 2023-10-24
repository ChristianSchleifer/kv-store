use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

use crate::server::command::Command;
use crate::store::KvStore;

mod command;

pub struct Server {
    listener: TcpListener,
    kv_store: KvStore,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            listener: TcpListener::bind(addr).expect("could not start server"),
            kv_store: KvStore::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let (stream, _) = self.listener.accept().expect("TCP connection failed");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, stream: TcpStream) {
        let buff_reader = BufReader::new(&stream);
        let mut buff_writer = BufWriter::new(&stream);

        for line in buff_reader.lines() {
            let line = line.expect("could not read line");
            let output = self.handle_line(line);
            buff_writer.write_all(output.as_bytes()).expect("could not write line");
            buff_writer.flush().expect("could not flush")
        }
    }


    fn handle_line(&mut self, line: String) -> String {
        match command::parse_command(line) {
            Ok(Command::GET(k)) => {
                let mut value = self.kv_store
                    .get(k)
                    .unwrap()
                    .unwrap_or("no value stored".to_string());
                value.push('\n');
                value
            }
            Ok(Command::SET(k, v)) => {
                self.kv_store.set(k, v).expect("IO error");
                "success\n".to_string()
            }
            Ok(Command::DELETE(k)) => {
                self.kv_store.delete(k).expect("IO error");
                "success\n".to_string()
            }
            Err(_) => "Unknown command\n".to_string()
        }
    }
}
