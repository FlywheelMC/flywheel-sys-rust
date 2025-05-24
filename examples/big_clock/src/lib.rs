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

        let mut batch_set = world.batch_set();

        for y in 0..10 {
            for x in 0..57 {
                batch_set.put(BlockPos::new(x, 0, y), Block::new("minecraft:tinted_glass"));
            }
        }

        for (i, ch,) in (Instant::now() - Duration::from_hours(4))
            .as_chrono().format("%H:%M:%S").to_string()
            .chars()
            .enumerate()
        {
            for (x, y,) in faces.get(&ch).unwrap() {
                let x = x + (7 * i) + 1;
                let y = y + 1;
                batch_set.put(BlockPos::new(x as _, 0, y as _), Block::new("minecraft:white_concrete"));
            }
        }

        batch_set.submit();
    }
}


async fn chunk_load(player : Player, chunk : ChunkPos) {
    player.world().mark_ready(chunk);
}
