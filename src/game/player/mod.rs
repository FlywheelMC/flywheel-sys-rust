use crate::prelude::*;


mod profile;
pub use profile::PlayerProfile;


unsafe extern "C" {
    unsafe fn flywheel_player_send_chat(session_id : u64, in_msg : u32, msg_len : u32);
    unsafe fn flywheel_player_send_actionbar(session_id : u64, in_msg : u32, msg_len : u32);
    unsafe fn flywheel_player_send_title(
        session_id     : u64,
        in_title       : u32,
        title_len      : u32,
        in_subtitle    : u32,
        subtitle_len   : u32,
        fade_in_secs   : u64,
        fade_in_nanos  : u32,
        stay_secs      : u64,
        stay_nanos     : u32,
        fade_out_secs  : u64,
        fade_out_nanos : u32
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


#[derive(Clone, Copy)]
pub struct Player {
    session_id : u64
}

impl Player {

    #[inline]
    pub unsafe fn from_session_id(session_id : u64) -> Self { Self { session_id } }

    #[inline]
    pub fn session_id(&self) -> u64 { self.session_id }

    pub fn profile(&self) -> Option<PlayerProfile> {
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

}

impl Player {

    pub fn send_chat(&self, msg : &str) {
        unsafe { flywheel_player_send_chat(self.session_id, msg.as_ptr() as u32, msg.len() as u32); }
    }

    pub fn send_actionbar(&self, msg : &str) {
        unsafe { flywheel_player_send_actionbar(self.session_id, msg.as_ptr() as u32, msg.len() as u32); }
    }

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
        fade_in.as_secs(), fade_in.subsec_nanos(),
        stay.as_secs(), stay.subsec_nanos(),
        fade_out.as_secs(), fade_out.subsec_nanos()
    ); } }

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
