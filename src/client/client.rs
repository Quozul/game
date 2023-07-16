use std::error::Error;
use std::net::SocketAddr;

use bevy::tasks::Task;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;
use quinn::{Connection, Endpoint};

use crate::server::server::server_addr;

static SERVER_NAME: &str = "localhost";

fn client_addr() -> SocketAddr {
    "127.0.0.1:5000".parse::<SocketAddr>().unwrap()
}

#[derive(Resource)]
pub(crate) struct ClientResource {
    connection: Option<Connection>,
}

impl ClientResource {
    async fn create_client(&mut self) {
        if let Ok(endpoint) = Endpoint::client(client_addr()) {
            if let Ok(connection) = endpoint.connect(server_addr(), SERVER_NAME)?.await {
                self.connection = Some(connection);
            }
        }
    }
}

impl Default for ClientResource {
    fn default() -> Self {
        ClientResource { connection: None }
    }
}

#[derive(Component)]
struct ClientTask(Task<Option<Connection>>);

fn client(mut client: Res<ClientResource>) {
    let thread_pool = AsyncComputeTaskPool::get();

    let _ = thread_pool.spawn(async move { client.create_client().await });
}
