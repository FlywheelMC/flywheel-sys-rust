use flywheel_sys::prelude::*;


#[unsafe(no_mangle)]
pub fn flywheel_main() {
    App::new()
        .on_player_joined(player_joined)
        .on_world_chunk_loading(load_chunk)
        .run();
}


async fn player_joined(player : Player) {
    if let Some(profile) = player.fetch_profile() {
        player.send_title(
            "<orange><b><u>Sine World</></></>",
            &format!("<yellow>Welcome, {}!</>", profile.name),
            Duration::ZERO, Duration::from_ticks(50), Duration::from_ticks(20)
        );
    }
}


const SINE_FREQ : f32 = 0.0625;
const SINE_AMP  : f32 = 5.0;
async fn load_chunk(player : Player, chunk : ChunkPos) {
    let mut blocks = Vec::new();
    let min = chunk.min_block();
    for dx in 0..16 {
        for dz in 0..16 {
            let x  = min.x + dx;
            let z  = min.z + dz;
            let hx = (x as f32) * SINE_FREQ;
            let hz = (z as f32) * SINE_FREQ;
            let h  = ((hx.sin() * hz.sin() + 1.0) * SINE_AMP) as i64;
            let mat = if (dx == 0 || dz == 0 || dx == 15 || dz == 15)
                { "minecraft:black_concrete" } else { "minecraft:white_concrete" };
            for y in 0..=h {
                blocks.push((BlockPos::new(x, y, z), Block::new(mat),));
            }
        }
    }
    //player.send_chat(&format!("<#ff0000>{:?}</>", chunk));
    let world = player.world();
    world.batch_set(blocks);
    world.mark_ready(chunk);
}
