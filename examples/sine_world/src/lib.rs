use flywheel_sys::prelude::*;


#[unsafe(no_mangle)]
pub fn flywheel_main() {
    App::new()
        .on_player_joined(player_joined)
        .on_world_chunk_loading(load_chunk)
        .on_world_chunk_unloaded(unload_chunk)
        .run();
}


async fn player_joined(player : Player) {
    if let Some(profile) = player.profile() {
        player.send_title(
            "<orange><b><u>Sine World</></></>",
            &format!("<yellow>Welcome, {}!</>", profile.name),
            Duration::ZERO, Duration::from_ticks(50), Duration::from_ticks(20)
        );
    }
}


const SINE_FREQ : f32 = 0.0625;
const SINE_AMP  : f32 = 5.0;
async fn load_chunk(player : Player, pos : ChunkPos) {
    player.send_chat(&format!("<grey>Loaded chunk ({}, {})</>", pos.x, pos.z));
    // let mut blocks = Vec::new();
    // let min = pos.min_block();
    // for dx in 0..16 {
    //     for dz in 0..16 {
    //         let x  = min.x + dx;
    //         let z  = min.z + dz;
    //         let hx = (x as f32) * SINE_FREQ;
    //         let hz = (z as f32) * SINE_FREQ;
    //         let h  = ((hx.sin() * hz.sin() + 1.0) * SINE_AMP) as i64;
    //         for y in 0..=h {
    //             blocks.push((BlockPos::new(x, y, z), Block::new("minecraft:stone"),));
    //         }
            
    //     }
    // }
    // player.world().batch_set(blocks);
    player.world().set(pos.min_block(), &Block::new("minecraft:stone"));
}

async fn unload_chunk(player : Player, pos : ChunkPos) {
    player.send_chat(&format!("<grey>Unloaded chunk ({}, {})</>", pos.x, pos.z));
}
