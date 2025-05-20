use crate::game::player::Player;
use crate::game::data::{ BlockPos, Block };
use core::mem;


unsafe extern "C" {
    unsafe fn flywheel_world_set_blocks(session_id : u64, in_data : u32);
}


/// A [`Player`]'s world.
///
/// Each player has their own world with its own blocks and entities.
pub struct World<'l> {
    pub(super) player : &'l Player
}

impl World<'_> {

    /// Set a single block in the world.
    ///
    /// This is an expensive operation. Consider using [`World::batch_set`] to set multiple block at once.
    #[inline]
    pub fn set(&self, pos : BlockPos, block : &Block) {
        self.batch_set([(pos, block,)])
    }


    /// Sets multiple blocks in the world.
    ///
    /// This is an expensive operation.
    pub fn batch_set<I, B>(&self, blocks : I)
    where
        I : IntoIterator<Item = (BlockPos, B,)>,
        B : AsRef<Block>
    {
        let mut data = vec![0u8; mem::size_of::<u32>()];
        let mut count = 0u32;
        for (pos, block,) in blocks {
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
            self.player.session_id,
            data.as_ptr() as u32
        ); }
    }

}
