use crate::uuid::Uuid;


unsafe extern "C" {
    pub(super) unsafe fn flywheel_profile_from_session(session_id : u64, out_uuid : u32, out_name_ptr : u32, out_name_len : u32) -> u32;
}


pub struct PlayerProfile {
    pub uuid : Uuid,
    pub name : String
}
