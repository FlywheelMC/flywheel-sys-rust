use crate::uuid::Uuid;


unsafe extern "C" {
    unsafe fn flywheel_rand_bool(out_u8 : u32);
    unsafe fn flywheel_rand_u8(out_u8 : u32);
    unsafe fn flywheel_rand_u16(out_u16 : u32);
    unsafe fn flywheel_rand_u32(out_u32 : u32);
    unsafe fn flywheel_rand_u64(out_u64 : u32);
    unsafe fn flywheel_rand_u128(out_u128 : u32);
    unsafe fn flywheel_rand_f32(out_f32 : u32);
    unsafe fn flywheel_rand_f64(out_f64 : u32);
}


pub trait GetRandom {
    fn random() -> Self;
}

impl GetRandom for bool {
    fn random() -> Self {
        let mut out = 0u8;
        unsafe { flywheel_rand_bool((&mut out) as (*mut _) as u32); }
        u8::from_le(out) != 0
    }
}

impl GetRandom for u8 {
    fn random() -> Self {
        let mut out = 0u8;
        unsafe { flywheel_rand_u8((&mut out) as (*mut _) as u32); }
        Self::from_le(out)
    }
}
impl GetRandom for i8 {
    fn random() -> Self { <u8 as GetRandom>::random().cast_signed() }
}

impl GetRandom for u16 {
    fn random() -> Self {
        let mut out = 0u16;
        unsafe { flywheel_rand_u16((&mut out) as (*mut _) as u32); }
        Self::from_le(out)
    }
}
impl GetRandom for i16 {
    fn random() -> Self { <u16 as GetRandom>::random().cast_signed() }
}

impl GetRandom for u32 {
    fn random() -> Self {
        let mut out = 0u32;
        unsafe { flywheel_rand_u32((&mut out) as (*mut _) as u32); }
        Self::from_le(out)
    }
}
impl GetRandom for i32 {
    fn random() -> Self { <u32 as GetRandom>::random().cast_signed() }
}

impl GetRandom for u64 {
    fn random() -> Self {
        let mut out = 0u64;
        unsafe { flywheel_rand_u64((&mut out) as (*mut _) as u32); }
        Self::from_le(out)
    }
}
impl GetRandom for i64 {
    fn random() -> Self { <u64 as GetRandom>::random().cast_signed() }
}

impl GetRandom for u128 {
    fn random() -> Self {
        let mut out = 0u128;
        unsafe { flywheel_rand_u128((&mut out) as (*mut _) as u32); }
        Self::from_le(out)
    }
}
impl GetRandom for i128 {
    fn random() -> Self { <u128 as GetRandom>::random().cast_signed() }
}

impl GetRandom for f32 {
    fn random() -> Self {
        let mut out = 0u32;
        unsafe { flywheel_rand_f32((&mut out) as (*mut _) as u32); }
        Self::from_bits(u32::from_le(out))
    }
}

impl GetRandom for f64 {
    fn random() -> Self {
        let mut out = 0u64;
        unsafe { flywheel_rand_f64((&mut out) as (*mut _) as u32); }
        Self::from_bits(u64::from_le(out))
    }
}

impl GetRandom for Uuid {
    fn random() -> Self {
        Uuid::from_u128(u128::random())
    }
}
