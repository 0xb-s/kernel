use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::{sync::atomic::AtomicU64, time::Duration};
#[derive(Copy, Clone, Debug)]
pub struct Jiffies(u64);

pub(crate) static ELAPSED: AtomicU64 = AtomicU64::new(0);

impl Jiffies {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn elapsed() -> Self {
        Self::new(ELAPSED.load(core::sync::atomic::Ordering::Relaxed))
    }

    pub fn to(self) -> u64 {
        self.0
    }

    pub fn reset() {
        ELAPSED.store(0, core::sync::atomic::Ordering::Relaxed);
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_millis(self.to())
    }
    pub fn update_elapsed(value: u64) {
        ELAPSED.store(value, core::sync::atomic::Ordering::Relaxed);
    }

    pub fn increment_elapsed(delta: u64) {
        ELAPSED.fetch_add(delta, core::sync::atomic::Ordering::Relaxed);
    }
}

impl From<Jiffies> for Duration {
    fn from(value: Jiffies) -> Self {
        value.as_duration()
    }
}
impl From<Duration> for Jiffies {
    fn from(duration: Duration) -> Self {
        Self(duration.as_millis() as u64)
    }
}
impl Add for Jiffies {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Jiffies {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl AddAssign for Jiffies {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for Jiffies {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl fmt::Display for Jiffies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} jiffies", self.0)
    }
}

impl PartialEq for Jiffies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Jiffies {}

impl PartialOrd for Jiffies {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Jiffies {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
