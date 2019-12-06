pub use core::time::Duration;

use core::ops::Sub;

/// An opaque point in time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    secs: u64,
    nanos: u32,
}

impl Instant {
    /// The current time.
    pub fn now() -> Instant {
        let mut secs = 0;
        let mut nanos = 0;

        unsafe {
            let ret = crate::intrinsics::current_time(&mut secs, &mut nanos);
            assert_eq!(ret, 0);
        }

        Instant { secs, nanos }
    }

    pub fn duration_since(self, earlier: Instant) -> Duration { self - earlier }

    /// How much time has elapsed since the [`Instant`] was created?
    pub fn elapsed(self) -> Duration { Instant::now().duration_since(self) }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        let this = Duration::new(self.secs, self.nanos);
        let other = Duration::new(other.secs, other.nanos);

        this - other
    }
}
