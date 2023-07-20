use std::thread;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::client::Client;

use shared::server::server::start_server_app;

use crate::client::join_server;
use crate::AppState;

#[derive(Default, Resource)]
pub struct UiState {
    label: String,
}

pub(crate) fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut client: ResMut<Client>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        let clicked = ui.button("Start server").clicked();

        if clicked {
            thread::spawn(|| {
                start_server_app();
            });
            ui_state.label = "127.0.0.1".to_string()
        }

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Server IP: ");
            ui.text_edit_singleline(&mut ui_state.label);

            if ui.button("Connect").clicked() {
                join_server(&mut next_state, &mut client, ui_state.label.as_str());
            }
        });
    });
}

pub(crate) fn display_network_stats(mut contexts: EguiContexts, client: ResMut<Client>) {
    let ctx = contexts.ctx_mut();

    if let Some(connection) = client.get_connection() && let Some(stats) = connection.stats() {
        egui::Window::new("Network stats").show(ctx, |ui| {
            ui.label(format!("udp_rx {} msgs", stats.udp_rx.datagrams));
            ui.label(format!("udp_tx {} msgs", stats.udp_tx.datagrams));
            ui.label(format!("ping {} ms", stats.frame_rx.ping));
        });
    }
}
