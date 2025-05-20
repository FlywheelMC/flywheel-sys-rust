#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x : i32,
    pub z : i32
}

impl ChunkPos {

    pub const ZERO : Self = Self::splat(0);

    #[inline]
    pub const fn new(x : i32, z : i32) -> Self { Self { x, z } }

    #[inline(always)]
    pub const fn splat(v : i32) -> Self { Self::new(v, v) }

    #[inline]
    pub const fn south(&self, offset : i32) -> Self { Self::new(self.x, self.z + offset) }
    #[inline]
    pub const fn north(&self, offset : i32) -> Self { Self::new(self.x, self.z - offset) }
    #[inline]
    pub const fn east(&self, offset : i32) -> Self { Self::new(self.x + offset, self.z) }
    #[inline]
    pub const fn west(&self, offset : i32) -> Self { Self::new(self.x - offset, self.z) }

    #[inline]
    pub const fn min_block(&self) -> BlockPos { BlockPos::new((self.x as i64) * 16, 0, (self.z as i64) * 16) }

}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x : i64,
    pub y : i64,
    pub z : i64
}

impl BlockPos {

    pub const ZERO : Self = Self::splat(0);

    #[inline]
    pub const fn new(x : i64, y : i64, z : i64) -> Self { Self { x, y, z } }

    #[inline(always)]
    pub const fn splat(v : i64) -> Self { Self::new(v, v, v) }

    #[inline]
    pub const fn south(&self, offset : i64) -> Self { Self::new(self.x, self.y, self.z + offset) }
    #[inline]
    pub const fn north(&self, offset : i64) -> Self { Self::new(self.x, self.y, self.z - offset) }
    #[inline]
    pub const fn up(&self, offset : i64) -> Self { Self::new(self.x, self.y + offset, self.z) }
    #[inline]
    pub const fn down(&self, offset : i64) -> Self { Self::new(self.x, self.y - offset, self.z) }
    #[inline]
    pub const fn east(&self, offset : i64) -> Self { Self::new(self.x + offset, self.y, self.z) }
    #[inline]
    pub const fn west(&self, offset : i64) -> Self { Self::new(self.x - offset, self.y, self.z) }

    #[inline]
    pub const fn chunk(&self) -> ChunkPos { ChunkPos::new((self.x / 16) as i32, (self.z / 16) as i32) }

}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundCategory {
    Master  = 0,
    Music   = 1,
    Records = 2,
    Weather = 3,
    Blocks  = 4,
    Hostile = 5,
    Neutral = 6,
    Players = 7,
    Ambient = 8,
    Voice   = 9
}
