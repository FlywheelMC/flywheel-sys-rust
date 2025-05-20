use crate::game::player::Player;
use crate::game::data::BlockPos;
use core::mem;
use std::collections::HashMap;


unsafe extern "C" {
    unsafe fn flywheel_world_set_blocks(session_id : u64, in_data : u32, data_len : u32);
}


pub struct World<'l> {
    pub(super) player : &'l Player
}

impl World<'_> {

    #[inline]
    pub fn set(&self, pos : BlockPos, block : &Block) {
        self.batch_set([(pos, block,)])
    }

    pub fn batch_set<'l, I>(&self, blocks : I)
    where
        I : IntoIterator<Item = (BlockPos, &'l Block,)>
    {
        let mut data = vec![0u8; mem::size_of::<u32>()];
        let mut count = 0u32;
        for (pos, block,) in blocks {
            count += 1;
            data.extend(pos.x.to_le_bytes());
            data.extend(pos.y.to_le_bytes());
            data.extend(pos.z.to_le_bytes());
            data.extend((block.id.len() as u32).to_le_bytes());
            data.extend(block.id.as_bytes());
            data.extend((block.states.len() as u8).to_le_bytes());
            for (state, value,) in &block.states {
                data.extend((state.len() as u32).to_le_bytes());
                data.extend(state.as_bytes());
                data.extend((value.len() as u32).to_le_bytes());
                data.extend(value.as_bytes());
            }
        }
        data[0..(mem::size_of::<u32>())].copy_from_slice(&count.to_le_bytes());
        unsafe { flywheel_world_set_blocks(
            self.player.session_id,
            data.as_ptr() as u32,
            data.len() as u32
        ); }
    }

}


pub struct Block {
    id     : String,
    states : HashMap<String, String>
}

impl Block {

    pub fn new(id : &str) -> Self {
        Self { id : id.to_string(), states : HashMap::new() }
    }

    #[inline]
    pub fn id(&self) -> &str { &self.id }

    pub fn set(&mut self, state : &str, value : &str) -> &mut Self {
        self.states.insert(state.to_string(), value.to_string());
        assert!(self.states.len() < 16);
        self
    }

    #[inline]
    pub fn with(mut self, state : &str, value : &str) -> Self {
        self.set(state, value);
        self
    }

    pub fn get(&self, state : &str) -> Option<&str> {
        self.states.get(state).map(|s| s.as_str())
    }

}
