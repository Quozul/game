use std::error::Error;
use std::net::SocketAddr;

use bevy::tasks::Task;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;
use quinn::{Endpoint, ServerConfig};

#[derive(Resource)]
pub(crate) struct ServerResource {
    endpoint: Option<Endpoint>,
}

impl Default for ServerResource {
    fn default() -> Self {
        ServerResource { endpoint: None }
    }
}

impl ServerResource {
    async fn create_server(&mut self) {
        if let Ok((certs, key)) = generate_self_signed_cert() {
            if let Ok(server_config) = ServerConfig::with_single_cert(vec![certs], key) {
                if let Ok(endpoint) = Endpoint::server(server_config, server_addr()) {
                    self.endpoint = Some(endpoint);
                }
            }
        }
    }
}

#[derive(Component)]
struct ServerTask(Task<Option<Endpoint>>);

pub(crate) fn server_addr() -> SocketAddr {
    "127.0.0.1:5001".parse::<SocketAddr>().unwrap()
}

fn generate_self_signed_cert() -> Result<(rustls::Certificate, rustls::PrivateKey), Box<dyn Error>>
{
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
    let key = rustls::PrivateKey(cert.serialize_private_key_der());
    Ok((rustls::Certificate(cert.serialize_der()?), key))
}

async fn create_server() -> Result<Endpoint, Box<dyn Error>> {
    let (certs, key) = generate_self_signed_cert()?;
    let server_config = ServerConfig::with_single_cert(vec![certs], key)?;
    let endpoint = Endpoint::server(server_config, server_addr())?;

    Ok(endpoint)
}

fn server(mut server: Res<ServerResource>) {
    let thread_pool = AsyncComputeTaskPool::get();

    let _ = thread_pool.spawn(async move { server.create_server().await });
}

async fn server_accept(endpoint: &Endpoint) -> Result<(), Box<dyn Error>> {
    if let Some(conn) = endpoint.accept().await {
        let connection = conn.await?;
        println!("{:?}", connection.local_ip());
    }

    Ok(())
}
