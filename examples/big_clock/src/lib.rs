#![feature(
    iter_array_chunks,
    iter_collect_into,
    duration_constructors_lite
)]


use flywheel_sys::prelude::*;
use std::collections::BTreeMap;


const CHARS : &str = "0123456789:";


#[unsafe(no_mangle)]
pub fn flywheel_main() {
    App::new()
        .on_start(plot_started)
        .on_world_chunk_loading(chunk_load)
        .run();
}


async fn plot_started() {
    let faces = include_str!("../faces.txt").lines()
        .array_chunks::<9>()
        .enumerate()
        .map(|(i, a)| (CHARS.chars().nth(i).unwrap(),
            a[..8].into_iter()
                .enumerate()
                .flat_map(|(y, b)| b.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| {
                        (c == '#').then(move || (x, y,))
                    })
                )
                .collect::<Vec<_>>()
        ))
        .collect::<BTreeMap<_, _>>();

    let player = unsafe { Player::from_session_id(0) };
    let world  = player.world();
    loop {
        task::sleep(Duration::from_ticks(10)).await;
        let mut blocks = Vec::new();

        let solid = (Instant::now() - Duration::from_hours(4))
            .as_chrono().format("%H:%M:%S").to_string()
            .chars()
            .enumerate()
            .flat_map(|(i, c,)| faces.get(&c).unwrap().iter()
                .map(move |(x, y,)| (x + (7*i), *y,))
            )
            .collect::<Vec<_>>();

        (0..8).into_iter()
            .flat_map(|y| (0..56).into_iter()
                .map(move |x| (x, y,))
            )
            .filter(|p| ! solid.contains(p))
            .map(|(x, y,)| (
                BlockPos::new(x as i64, 0, y as i64),
                Block::new("minecraft:air"),
            ))
            .collect_into(&mut blocks);

        solid
            .into_iter()
            .map(|(x, y,)| (
                BlockPos::new(x as i64, 0, y as i64),
                Block::new("minecraft:black_concrete"),
            ))
            .collect_into(&mut blocks);

        debug!("Update");
        world.batch_set(blocks);
    }
}


async fn chunk_load(player : Player, chunk : ChunkPos) {
    debug!("Loading {:?}", chunk);
    player.world().mark_ready(chunk);
}
