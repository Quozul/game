#![feature(async_closure)]

use crate::menu::setup_menu;
use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::{Client, QuinnetClientPlugin};
use std::net::{IpAddr, Ipv4Addr};

use crate::server::server::ServerResource;

mod client;
mod menu;
mod server;

fn main() {
    App::new()
        .insert_resource(ServerResource::default())
        .add_plugins((DefaultPlugins, QuinnetClientPlugin::default()))
        .add_systems(Startup, setup_menu)
        .run();
}

fn start_connection(mut client: ResMut<Client>) {
    client
        .open_connection(
            ConnectionConfiguration::from_ips(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                6000,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                0,
            ),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}
