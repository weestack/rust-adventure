mod client;
mod server;
mod utils;



fn main() -> std::io::Result<()> {
    // initialize
    let server = server::ReverseShellServer::new("127.0.0.1:9999");
    let client = client::ReverseClient::new("127.0.0.1:9999");

    // get thread handles
    let server_handle = server.init_reverse_shell();
    let client_handle = client.init_shell();

    // wait for the execution to finish
    server_handle.join().unwrap();
    client_handle.join().unwrap();

    Ok(())
}