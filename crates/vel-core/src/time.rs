use time::OffsetDateTime;

pub trait Clock {
    fn now(&self) -> OffsetDateTime;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedClock {
    now: OffsetDateTime,
}

impl FixedClock {
    pub const fn new(now: OffsetDateTime) -> Self {
        Self { now }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> OffsetDateTime {
        self.now
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn fixed_clock_returns_supplied_time() {
        let now = datetime!(2026-03-17 15:45:00 UTC);
        let clock = FixedClock::new(now);

        assert_eq!(clock.now(), now);
    }
}
