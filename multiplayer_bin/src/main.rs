use multiplayer::client::Client;
use multiplayer::server::Server;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Text { content: String },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8080";
    let server = start_server(server_address);
    let client = start_client(server_address);

    match tokio::try_join!(server, client) {
        Err(e) => println!("an error occurred; error = {:?}", e),
        _ => println!("done!"),
    }

    Ok(())
}

async fn start_server(server_address: &str) -> io::Result<()> {
    println!("[Server] Starting server on {}", server_address);

    let server = Server::new(server_address).await?;
    server.listen().await?;

    Ok(())
}

async fn start_client(server_address: &str) -> io::Result<()> {
    println!("[Client] Connecting to server at {}", server_address);

    let mut client = Client::connect(server_address).await?;

    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;

        let message = Message::Text {
            content: buffer.trim().to_string(),
        };

        client.send_serialize(&message).await.unwrap();

        match client.read_serialize::<Message>().await {
            Ok(message) => {
                println!("[Client] Received message: {:?}", message);
            }
            Err(err) => {
                eprintln!("[Client] an error occurred; error = {:?}", err)
            }
        }
    }
}
