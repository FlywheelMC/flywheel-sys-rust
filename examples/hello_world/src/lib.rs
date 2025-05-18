use flywheel_sys::prelude::*;


#[unsafe(no_mangle)]
pub fn flywheel_main() {
    App::new()
        .on_start(plot_started)
        .run();
}


async fn plot_started() {
    pass!("WASM is running!");
}
