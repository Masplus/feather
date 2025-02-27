use std::sync::Arc;

use base::{Chunk, ChunkPosition};
use parking_lot::RwLock;

use crate::view::View;

mod block_change;

pub use block_change::BlockChangeEvent;
/// Triggered when a player joins the `Game`.
#[derive(Debug)]
pub struct PlayerJoinEvent;

/// Event triggered when a player changes their `View`,
/// meaning they crossed into a new chunk.
#[derive(Debug)]
pub struct ViewUpdateEvent {
    pub old_view: View,
    pub new_view: View,

    /// Chunks that are in `new_view` but not `old_view`
    pub new_chunks: Vec<ChunkPosition>,
    /// Chunks that are in `old_view` but not in `new_view`
    pub old_chunks: Vec<ChunkPosition>,
}

impl ViewUpdateEvent {
    pub fn new(old_view: View, new_view: View) -> Self {
        let mut this = Self {
            old_view,
            new_view,
            new_chunks: new_view.difference(old_view).collect(),
            old_chunks: old_view.difference(new_view).collect(),
        };
        this.new_chunks
            .sort_unstable_by_key(|chunk| chunk.distance_squared_to(new_view.center()));
        this.old_chunks
            .sort_unstable_by_key(|chunk| chunk.distance_squared_to(old_view.center()));
        this
    }
}

/// Event triggered when an entity crosses into a new chunk.
///
/// Unlike [`ViewUpdateEvent`], this event triggers for all entities,
/// not just players.
pub struct ChunkCrossEvent {
    pub old_chunk: ChunkPosition,
    pub new_chunk: ChunkPosition,
}

/// Triggered when a chunk is loaded.
#[derive(Debug)]
pub struct ChunkLoadEvent {
    pub position: ChunkPosition,
    pub chunk: Arc<RwLock<Chunk>>,
}

/// Triggered when an error occurs while loading a chunk.
#[derive(Debug)]
pub struct ChunkLoadFailEvent {
    pub position: ChunkPosition,
}

/// Triggered when an entity is removed from the world.
///
/// The entity will remain alive for one tick after it is
/// destroyed to allow systems to observe this event.
#[derive(Debug)]
pub struct EntityRemoveEvent;

/// Triggered when an entity is added into the world.
#[derive(Debug)]
pub struct EntityCreateEvent;
