use flywheel_sys::prelude::*;


#[unsafe(no_mangle)]
pub fn flywheel_main() {
    App::new()
        .on_start(plot_started)
        .on_player_joined(player_joined)
        .run();
}


async fn plot_started() {
    pass!("WASM is running!");
}


async fn player_joined(player : Player) {
    if let Some(profile) = player.profile() {
        player.send_chat(&format!("<green>Hello, {}!</green>", profile.name));
    }
}
