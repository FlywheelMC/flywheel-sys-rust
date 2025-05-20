//! Game-related data structures and operations.


mod server;
pub use server::Server;
mod player;
pub use player::Player;

pub mod data;


/// Commonly used items.
pub mod prelude {
    pub use super::server::Server;
    pub use super::player::{ Player, World };

    pub use super::data::{ ChunkPos, BlockPos, SoundCategory, Block };
}
