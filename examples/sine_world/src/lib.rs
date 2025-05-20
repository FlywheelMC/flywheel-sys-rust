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


async fn load_chunk(player : Player, pos : ChunkPos) {
    player.send_chat(&format!("<grey>Loaded chunk ({}, {})</>", pos.x, pos.z));
    player.world().set(pos.min_block(), &Block::new("minecraft:stone"));
}

async fn unload_chunk(player : Player, pos : ChunkPos) {
    player.send_chat(&format!("<grey>Unloaded chunk ({}, {})</>", pos.x, pos.z));
}
