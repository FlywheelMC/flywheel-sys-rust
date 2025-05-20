//! Data structures which don't do anything by themselves,
//!  but are used for certian operations.


use std::collections::HashMap;


/// A chunk position in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    /// The x position of this `ChunkPos`.
    pub x : i32,
    /// The z position of this `ChunkPos`.
    pub z : i32
}

impl ChunkPos {

    /// The chunk at the world origin (`<0, 0>`).
    pub const ZERO : Self = Self::splat(0);

    /// Create a new `ChunkPos` with the given `x` and `z` values.
    #[inline]
    pub const fn new(x : i32, z : i32) -> Self { Self { x, z } }

    /// Create a new `ChunkPos` with the given value as `x` and `z`.
    #[inline(always)]
    pub const fn splat(v : i32) -> Self { Self::new(v, v) }

    /// Returns a new `ChunkPos` shifted `offset` chunks to the south (positive z).
    #[inline]
    pub const fn south(&self, offset : i32) -> Self { Self::new(self.x, self.z + offset) }

    /// Returns a new `ChunkPos` shifted `offset` chunks to the north (negative z).
    #[inline]
    pub const fn north(&self, offset : i32) -> Self { Self::new(self.x, self.z - offset) }

    /// Returns a new `ChunkPos` shifted `offset` chunks to the east (positive x).
    #[inline]
    pub const fn east(&self, offset : i32) -> Self { Self::new(self.x + offset, self.z) }

    /// Returns a new `ChunkPos` shifted `offset` chunks to the west (negative x).
    #[inline]
    pub const fn west(&self, offset : i32) -> Self { Self::new(self.x - offset, self.z) }

    /// The minimum (most negative `x`, `y`, and `z`) `BlockPos` in this chunk.
    #[inline]
    pub const fn min_block(&self) -> BlockPos { BlockPos::new((self.x as i64) * 16, 0, (self.z as i64) * 16) }

}


/// A block position in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockPos {
    /// The x position of this `BlockPos`.
    pub x : i64,
    /// The y position of this `BlockPos`.
    pub y : i64,
    /// The z position of this `BlockPos`.
    pub z : i64
}

impl BlockPos {

    /// The block at the world origin (`<0, 0, 0>`).
    pub const ZERO : Self = Self::splat(0);

    /// Create a new `BlockPos` with the given `x`, `y`, and `z` values.
    #[inline]
    pub const fn new(x : i64, y : i64, z : i64) -> Self { Self { x, y, z } }

    /// Create a new `BlockPos` with the given value as `x`, `y` and `z`.
    #[inline(always)]
    pub const fn splat(v : i64) -> Self { Self::new(v, v, v) }

    /// Returns a new `BlockPos` shifted `offset` blocks to the south (positive z).
    #[inline]
    pub const fn south(&self, offset : i64) -> Self { Self::new(self.x, self.y, self.z + offset) }

    /// Returns a new `BlockPos` shifted `offset` blocks to the north (negative z).
    #[inline]
    pub const fn north(&self, offset : i64) -> Self { Self::new(self.x, self.y, self.z - offset) }

    /// Returns a new `BlockPos` shifted `offset` blocks up (positive y).
    #[inline]
    pub const fn up(&self, offset : i64) -> Self { Self::new(self.x, self.y + offset, self.z) }

    /// Returns a new `BlockPos` shifted `offset` blocks down (negative y).
    #[inline]
    pub const fn down(&self, offset : i64) -> Self { Self::new(self.x, self.y - offset, self.z) }

    /// Returns a new `BlockPos` shifted `offset` blocks to the east (positive x).
    #[inline]
    pub const fn east(&self, offset : i64) -> Self { Self::new(self.x + offset, self.y, self.z) }

    /// Returns a new `BlockPos` shifted `offset` blocks to the west (negative x).
    #[inline]
    pub const fn west(&self, offset : i64) -> Self { Self::new(self.x - offset, self.y, self.z) }

    /// Gets the `ChunkPos` that this `BlockPos` falls in.
    #[inline]
    pub const fn chunk(&self) -> ChunkPos { ChunkPos::new((self.x / 16) as i32, (self.z / 16) as i32) }

}


/// The category that a played sound falls into.
///
/// Each category has its own volume slider in the player's option menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundCategory {
    /// The `Master` volume control also affects the volumes of the other categories.
    /// Generally, only use `Master` for the most import sounds.
    Master  = 0,
    /// `Music` is used for the in-game music.
    /// A lot of players have this category muted. Consider using `Records` instead.
    Music   = 1,
    /// `Records` is used for note blocks and jukeboxes.
    /// This is a good substitute for `Music`.
    Records = 2,
    /// `Weather` is used for rain and thunder.
    Weather = 3,
    /// `Blocks` is used for block placement, breaking, and other block behaviours.
    Blocks  = 4,
    /// `Hostile` is used for hostile mobs for idle, attack, movement, and other actions.
    Hostile = 5,
    /// `Neutral` is used for neutral mobs for idle, attack, movement, and other actions.
    Neutral = 6,
    /// `Players` is used for players actions.
    Players = 7,
    /// `Ambient` is used for ambient noises such as cave noises.
    Ambient = 8,
    /// TODO: Doc comment
    Voice   = 9
}


/// A world block, including material and states.
pub struct Block {
    id     : String,
    states : HashMap<String, String>
}

impl Block {

    /// Create a new block from a given block material ID.
    pub fn new(id : &str) -> Self {
        Self { id : id.to_string(), states : HashMap::new() }
    }

    /// Returns the material ID of this block.
    #[inline]
    pub fn id(&self) -> &str { &self.id }

    /// Sets a property of this block.
    /// 
    /// For a variant of this function which returns `self` see [`Self::with`].
    pub fn set(&mut self, state : &str, value : &str) -> &mut Self {
        self.states.insert(state.to_string(), value.to_string());
        assert!(self.states.len() < 16);
        self
    }

    /// Sets a property of this block.
    /// 
    /// For a variant of this function which mutates `self` see [`Self::set`].
    #[inline]
    pub fn with(mut self, state : &str, value : &str) -> Self {
        self.set(state, value);
        self
    }

    /// Returns a property of this block if set.
    pub fn get(&self, state : &str) -> Option<&str> {
        self.states.get(state).map(|s| s.as_str())
    }

    /// Returns the number of states set on this block.
    pub fn states_len(&self) -> usize {
        self.states.len()
    }

    /// Returns an iterator over this block's states.
    pub fn states(&self) -> impl Iterator<Item = (&str, &str,)> {
        self.states.iter().map(|(a, b,)| (a.as_str(), b.as_str(),))
    }

}

