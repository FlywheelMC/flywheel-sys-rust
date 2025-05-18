use core::ops::{ Add, AddAssign, Sub, SubAssign };
pub use core::time::Duration;


unsafe extern "C" {
    unsafe fn flywheel_system_dur_since_epoch( out_secs : u32, out_nanos : u32 );
}


#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant {
    after_epoch : Duration
}

impl Instant {

    pub const UNIX_EPOCH : Self = Self { after_epoch : Duration::ZERO };

    pub fn now() -> Self {
        let mut secs  = 0u64;
        let mut nanos = 0u32;
        unsafe { flywheel_system_dur_since_epoch((&mut secs) as (*mut _) as u32, (&mut nanos) as (*mut _) as u32); };
        Self { after_epoch : Duration::new(secs, nanos) }
    }

    pub fn duration_since(&self, earlier : Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    #[inline]
    pub fn checked_duration_since(&self, earlier : Instant) -> Option<Duration> {
        self.after_epoch.checked_sub(earlier.after_epoch)
    }

    #[inline]
    pub fn saturating_duration_since(&self, earlier : Instant) -> Duration {
        self.after_epoch.saturating_sub(earlier.after_epoch)
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().saturating_duration_since(*self)
    }

    pub fn checked_add(&self, duration : Duration) -> Option<Self> {
        Some(Self { after_epoch : self.after_epoch.checked_add(duration)? })
    }

    pub fn checked_sub(&self, duration : Duration) -> Option<Self> {
        Some(Self { after_epoch : self.after_epoch.checked_sub(duration)? })
    }

}

impl Add<Duration> for Instant {
    type Output = Self;
    fn add(self, other : Duration) -> Self {
        self.checked_add(other).expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, other : Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;
    fn sub(self, other : Duration) -> Self {
        self.checked_sub(other).expect("overflow when subtracting duration to instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other : Duration) {
        *self = *self - other;
    }
}
