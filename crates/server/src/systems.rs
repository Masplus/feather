//! Systems linking a `Server` and a `Game`.

mod entity;
mod player_join;
mod player_leave;
mod tablist;
pub mod view;

use std::time::{Duration, Instant};

use common::{Game, Name};
use ecs::{SysResult, SystemExecutor};

use crate::{client::ClientId, Server};

/// Registers systems for a `Server` with a `Game`.
pub fn register(server: Server, game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(server);

    player_join::register(systems);
    systems
        .group::<Server>()
        .add_system(handle_packets)
        .add_system(send_keepalives);
    view::register(game, systems);
    crate::chunk_subscriptions::register(systems);
    player_leave::register(systems);
    tablist::register(systems);
    entity::register(game, systems);
}

/// Polls for packets received from clients
/// and handles them.
fn handle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut packets = Vec::new();

    for (player, &client_id) in game.ecs.query::<&ClientId>().iter() {
        if let Some(client) = server.clients.get(client_id) {
            for packet in client.received_packets() {
                packets.push((player, packet));
            }
        }
    }

    for (player, packet) in packets {
        if let Err(e) = crate::packet_handlers::handle_packet(game, server, player, packet) {
            log::warn!(
                "Failed to handle packet from '{}': {:?}",
                &**game.ecs.get::<Name>(player)?,
                e
            );
        }
    }

    Ok(())
}

/// Sends out keepalive packets at an interval.
fn send_keepalives(_game: &mut Game, server: &mut Server) -> SysResult {
    let interval = Duration::from_secs(5);
    if server.last_keepalive_time + interval < Instant::now() {
        server.broadcast_keepalive();
    }
    Ok(())
}
