use crate::game::player::Player;
use core::marker::Tuple;
use core::pin::Pin;
use core::task::{ Context, Poll };
use wasm_rs_async_executor::single_threaded as task;


unsafe extern "C" {
    safe fn flywheel_refuel();
    unsafe fn flywheel_next_event(out_id_ptr : u32, out_id_len : u32, out_args_ptr : u32, out_args_len : u32) -> u32;
}


type EventCallbacks<Args> = Vec<Box<dyn Fn<Args, Output = Pin<Box<dyn Future<Output = ()>>>>>>;
#[derive(Default)]
pub struct App {
    on_start         : EventCallbacks<()>,
    //on_stopping      : EventCallbacks<()>,
    on_player_joined : EventCallbacks<(Player,)>,
    on_player_left   : EventCallbacks<(Player,)>
}

impl App {

    pub fn new() -> Self { Self::default() }

    pub fn run(&mut self) {
        task::block_on(async {
            Self::fire(&self.on_start, ());
            AppRunFuture { app : self }.await;
        })
    }

    fn fire<T : Tuple + Clone>(callbacks : &EventCallbacks<T>, args : T) {
        for callback in callbacks {
            task::spawn(callback.call(args.clone()));
        }
    }

}

macro event_fn( $ident:ident ( $( $argident:ident : $argty:ty ),* $(,)? ) ) {
    pub fn $ident<F, Fut>(&mut self, f : F) -> &mut Self
    where
        F   : (Fn( $( $argty , )* ) -> Fut) + 'static,
        Fut : Future<Output = ()> + 'static
    {
        self.$ident.push(Box::new(move | $( $argident , )* | Box::pin(f( $( $argident , )* ))));
        self
    }
}

impl App {
    event_fn!{ on_start(,) }
    event_fn!{ on_player_joined(player : Player,) }
    event_fn!{ on_player_left(player : Player,) }
}


struct AppRunFuture<'l> {
    app : &'l mut App
}

impl Future for AppRunFuture<'_> {
    type Output = !;
    fn poll(self : Pin<&mut Self>, _ : &mut Context<'_>) -> Poll<Self::Output> {
        let mut id_ptr   = 0u32;
        let mut id_len   = 0u32;
        let mut args_ptr = 0u32;
        let mut args_len = 0u32;
        if (unsafe { flywheel_next_event(
            (&mut id_ptr) as (*mut _) as u32,
            (&mut id_len) as (*mut _) as u32,
            (&mut args_ptr) as (*mut _) as u32,
            (&mut args_len) as (*mut _) as u32
        ) } != 0) {
            let id_len   = id_len as usize;
            let id       = unsafe { String::from_raw_parts(id_ptr as (*mut u8), id_len, id_len) };
            let args_len = args_len as usize;
            let args     = unsafe { Vec::from_raw_parts(args_ptr as (*mut u8), args_len, args_len) };
            match (id.as_str()) {

                "flywheel_player_join" => {
                    let session_id = u64::from_ne_bytes(*unsafe { args.as_chunks_unchecked::<8>().get_unchecked(0) });
                    let player     = unsafe { Player::from_session_id(session_id) };
                    App::fire(&self.app.on_player_joined, (player,))
                },

                "flywheel_player_left" => {
                    let session_id = u64::from_ne_bytes(*unsafe { args.as_chunks_unchecked::<8>().get_unchecked(0) });
                    let player     = unsafe { Player::from_session_id(session_id) };
                    App::fire(&self.app.on_player_left, (player,))
                },

                _ => {
                    #[cfg(feature = "selfhosted")]
                    crate::selfhosted::error!("Unknown event {:?} triggered", id);
                }
            }
        }
        flywheel_refuel();
        Poll::Pending
    }
}
