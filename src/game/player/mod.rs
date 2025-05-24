use crate::prelude::*;


mod profile;
pub use profile::PlayerProfile;

mod world;
pub use world::World;


unsafe extern "C" {
    safe fn flywheel_player_exists(session_id : u64) -> u32;
    unsafe fn flywheel_player_send_chat(session_id : u64, in_msg : u32, msg_len : u32);
    unsafe fn flywheel_player_send_actionbar(session_id : u64, in_msg : u32, msg_len : u32);
    unsafe fn flywheel_player_send_title(
        session_id   : u64,
        in_title     : u32,
        title_len    : u32,
        in_subtitle  : u32,
        subtitle_len : u32,
        fade_in      : u32,
        stay         : u32,
        fade_out     : u32
    );
    unsafe fn flywheel_player_send_sound(
        session_id : u64,
        in_id      : u32,
        id_len     : u32,
        category   : u32,
        volume     : f32,
        pitch      : f32,
        seed       : u64
    );
}


/// A [`Player`] on the server.
///
/// A [`Player`] can become useless at any time if the player leaves.
///  To check if a player is still on the server, use [`Player::exists`].
#[derive(Clone, Copy)]
pub struct Player {
    session_id : u64
}

impl Player {

    /// Creates a new [`Player`] from a session ID.
    ///
    /// ### Safety
    /// This function is not inherently unsafe.
    /// However, running operations on a [`Player`] with an invalid session ID waste time.
    #[inline]
    pub unsafe fn from_session_id(session_id : u64) -> Self { Self { session_id } }

    /// Gets this player's session ID.
    #[inline]
    pub fn session_id(&self) -> u64 { self.session_id }

    /// Checks if this [`Player`] is still in the server.
    pub fn exists(&self) -> bool {
        flywheel_player_exists(self.session_id) != 0
    }

    /// Requests this player's profile.
    ///
    /// If this player is no longer on the server, `None` is returned.
    pub fn fetch_profile(&self) -> Option<PlayerProfile> {
        let mut name_ptr = 0u32;
        let mut name_len = 0u32;
        let mut uuid     = 0u128;
        if (unsafe { profile::flywheel_profile_from_session(
            self.session_id,
            (&mut uuid) as (*mut _) as u32,
            (&mut name_ptr) as (*mut _) as u32,
            (&mut name_len) as (*mut _) as u32
        ) } == 0) { None } else {
            let name_len = name_len as usize;
            let name = unsafe { String::from_raw_parts(name_ptr as (*mut u8), name_len, name_len) };
            Some(PlayerProfile { uuid : Uuid::from_u128_le(uuid), name })
        }
    }

    /// Access to the player's world.
    pub fn world(&self) -> World<'_> {
        World { player : self }
    }

}

impl Player {

    /// Adds a chat message to the player's chat stream.
    ///  `msg` is in the XML text format.
    ///
    /// The chat appears on the left side of the player's screen.
    pub fn send_chat(&self, msg : &str) {
        unsafe { flywheel_player_send_chat(self.session_id, msg.as_ptr() as u32, msg.len() as u32); }
    }

    /// Show an actionbar message to the player.
    ///  `msg` is in the XML text format.
    ///
    /// The actionbar appears above the player's hotbar.
    pub fn send_actionbar(&self, msg : &str) {
        unsafe { flywheel_player_send_actionbar(self.session_id, msg.as_ptr() as u32, msg.len() as u32); }
    }

    /// Show a title message to the player.
    ///  `title` and `subtitle` are in the XML text format.
    ///
    /// The title appears in the middle of the player's screen.
    pub fn send_title(&self,
        title    : &str,
        subtitle : &str,
        fade_in  : Duration,
        stay     : Duration,
        fade_out : Duration
    ) { unsafe { flywheel_player_send_title(
        self.session_id,
        title.as_ptr() as u32, title.len() as u32,
        subtitle.as_ptr() as u32, subtitle.len() as u32,
        fade_in.as_ticks(),
        stay.as_ticks(),
        fade_out.as_ticks()
    ); } }

    /// Play a sound to the player.
    pub fn send_sound(&self,
        id       : &str,
        category : SoundCategory,
        volume   : f32,
        pitch    : f32,
        seed     : u64
    ) { unsafe { flywheel_player_send_sound(
        self.session_id,
        id.as_ptr() as u32, id.len() as u32,
        category as u32,
        volume, pitch, seed
    ); } }

}
