use crate::game::data::{ BlockPos, ChunkPos, Block };
use core::mem;
use std::collections::BTreeMap;


unsafe extern "C" {
    safe fn flywheel_world_mark_ready(session_id : u64, x : i32, z : i32);
    unsafe fn flywheel_world_set_blocks(session_id : u64, in_data : u32);
}


/// A [`Player`]'s world.
///
/// Each player has their own world with its own blocks and entities.
#[derive(Clone, Copy)]
pub struct World {
    pub(super) session_id : u64
}

impl World {

    /// Marks a chunk as ready to load.
    pub fn mark_ready(&self, chunk : ChunkPos) {
        flywheel_world_mark_ready(self.session_id, chunk.x, chunk.z);
    }

    /// Set a single block in the world.
    ///
    /// This is an expensive operation. Consider using [`World::batch_set`] to set multiple block at once.
    pub fn set(&self, pos : BlockPos, block : Block) {
        self.batch_set()
            .with(pos, block)
            .submit();
    }


    /// Sets multiple blocks in the world.
    ///
    /// This is an expensive operation.
    #[inline]
    pub fn batch_set(&self) -> BatchSet {
        BatchSet { session_id : self.session_id, blocks : BTreeMap::new() }
    }

}


pub struct BatchSet {
    session_id : u64,
    blocks     : BTreeMap<BlockPos, Block>
}

impl BatchSet {

    pub fn put(&mut self, pos : BlockPos, block : Block) {
        self.blocks.insert(pos, block);
    }

    pub fn with(mut self, pos : BlockPos, block : Block) -> Self {
        self.put(pos, block);
        self
    }


    pub fn submit(self) {
        let mut data = vec![0u8; mem::size_of::<u32>()];
        let mut count = 0u32;
        for (pos, block,) in self.blocks {
            let block = block.as_ref();
            count += 1;
            data.extend(pos.x.to_le_bytes());
            data.extend(pos.y.to_le_bytes());
            data.extend(pos.z.to_le_bytes());
            data.extend((block.id().len() as u32).to_le_bytes());
            data.extend(block.id().as_bytes());
            data.extend((block.states_len() as u8).to_le_bytes());
            for (state, value,) in block.states() {
                data.extend((state.len() as u32).to_le_bytes());
                data.extend(state.as_bytes());
                data.extend((value.len() as u32).to_le_bytes());
                data.extend(value.as_bytes());
            }
        }
        if (count == 0) { return; }
        data[0..(mem::size_of::<u32>())].copy_from_slice(&count.to_le_bytes());
        unsafe { flywheel_world_set_blocks(
            self.session_id,
            data.as_ptr() as u32
        ); }
    }

}
