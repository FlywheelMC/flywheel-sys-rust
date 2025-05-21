//! Reimplementations and extensions to [`std::time`].


use core::ops::{ Add, AddAssign, Sub, SubAssign };
pub use core::time::Duration;


unsafe extern "C" {
    unsafe fn flywheel_system_dur_since_epoch( out_secs : u32, out_nanos : u32 );
}


/// A reimplementation of [`std::time::Instant`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html) compatible with the Flywheel WASM API.
#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant {
    after_epoch : Duration
}

impl Instant {

    /// An anchor in time.
    ///
    /// This constant is defined to be “1970-01-01 00:00:00 UTC” on all systems
    ///
    /// See [`SystemTime::UNIX_EPOCH`](https://doc.rust-lang.org/stable/std/time/struct.SystemTime.html#associatedconstant.UNIX_EPOCH).
    pub const UNIX_EPOCH : Self = Self { after_epoch : Duration::ZERO };

    /// See [`Instant::now`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.now).
    pub fn now() -> Self {
        let mut secs  = 0u64;
        let mut nanos = 0u32;
        unsafe { flywheel_system_dur_since_epoch((&mut secs) as (*mut _) as u32, (&mut nanos) as (*mut _) as u32); };
        Self { after_epoch : Duration::new(secs, nanos) }
    }

    /// See [`Instant::duration_since`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.duration_since).
    pub fn duration_since(&self, earlier : Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// See [`Instant::checked_duration_since`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.checked_duration_since).
    #[inline]
    pub fn checked_duration_since(&self, earlier : Instant) -> Option<Duration> {
        self.after_epoch.checked_sub(earlier.after_epoch)
    }

    /// See [`Instant::saturating_duration_since`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.saturating_duration_since).
    #[inline]
    pub fn saturating_duration_since(&self, earlier : Instant) -> Duration {
        self.after_epoch.saturating_sub(earlier.after_epoch)
    }

    /// See [`Instant::elapsed`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.elapsed).
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().saturating_duration_since(*self)
    }

    /// See [`Instant::checked_add`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.checked_add).
    pub fn checked_add(&self, duration : Duration) -> Option<Self> {
        Some(Self { after_epoch : self.after_epoch.checked_add(duration)? })
    }

    /// See [`Instant::checked_sub`](https://doc.rust-lang.org/stable/std/time/struct.Instant.html#method.checked_sub).
    pub fn checked_sub(&self, duration : Duration) -> Option<Self> {
        Some(Self { after_epoch : self.after_epoch.checked_sub(duration)? })
    }

    /// Converts this `Instant` to a [`chrono::DateTime<Utc>`](chrono::DateTime).
    #[cfg(any(doc, feature = "chrono"))]
    #[doc(cfg(feature = "chorno"))]
    pub fn as_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::UNIX_EPOCH + self.after_epoch
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


/// Extends [`Duration`] with extra methods and constants for game tick lengths.
pub trait DurationExt {

    /// The duration of one game tick.
    ///
    /// One game tick is equivalent to 50 milliseconds, or 0.05 seconds.
    const TICK : Self;

    /// Creates a new `Duration` from the specified number of game ticks.
    ///
    /// One game tick is equivalent to 50 milliseconds, or 0.05 seconds.
    fn from_ticks(ticks : u32) -> Self;

    /// Returns the total number of whole game ticks contained by this `Duration`.
    ///
    /// One game tick is equivalent to 50 milliseconds, or 0.05 seconds.
    fn as_ticks(&self) -> u32;

    /// Returns the fractional part of this `Duration`, in whole game ticks.
    ///
    /// This method does **not** return the length of the duration when represented by milliseconds.
    //   The returned number always represents a fractional portion of a second (i.e., it is less than 20).
    ///
    /// One game tick is equivalent to 50 milliseconds, or 0.05 seconds.
    fn subsec_ticks(&self) -> u32;

}

impl DurationExt for Duration {

    const TICK : Self = Self::from_millis(50);

    fn from_ticks(ticks : u32) -> Self {
        Self::from_millis((ticks as u64) * 50)
    }

    fn as_ticks(&self) -> u32 {
        (self.as_millis() / 50) as u32
    }

    fn subsec_ticks(&self) -> u32 {
        self.subsec_millis() / 50
    }

}
