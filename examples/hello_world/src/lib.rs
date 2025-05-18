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
    info!("WASM detected session {} joined", player.session_id());
    if let Some(profile) = player.profile() {
        player.send_chat(&format!("<green>Hello, {}!</green>\n <yellow>Your UUID is {}.", profile.name, profile.uuid));
    }
}
