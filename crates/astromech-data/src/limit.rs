//! Spacing between requests to the same service.

use crate::Service;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Hands out request slots at least a service's interval apart. Each caller reserves its own
/// slot, so concurrent callers queue rather than burst.
#[derive(Debug, Clone)]
pub struct Limiter {
    interval: HashMap<Service, Duration>,
    next: HashMap<Service, Instant>,
}

impl Default for Limiter {
    /// Each service's [`Service::min_interval`].
    fn default() -> Self {
        Limiter { interval: Service::ALL.iter().map(|s| (*s, s.min_interval())).collect(), next: HashMap::new() }
    }
}

impl Limiter {
    /// No spacing at all (tests).
    pub fn none() -> Self {
        Limiter { interval: HashMap::new(), next: HashMap::new() }
    }

    pub fn interval(mut self, service: Service, gap: Duration) -> Self {
        self.interval.insert(service, gap);
        self
    }

    /// Reserve the next slot for `service`; returns how long to wait from `now` before sending.
    pub fn reserve(&mut self, service: Service, now: Instant) -> Duration {
        let gap = self.interval.get(&service).copied().unwrap_or_default();
        let at = self.next.get(&service).copied().filter(|t| *t > now).unwrap_or(now);
        self.next.insert(service, at + gap);
        at - now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_spaces_requests_per_service() {
        let mut l = Limiter::none().interval(Service::Edsm, Duration::from_secs(1));
        let t = Instant::now();
        assert_eq!(l.reserve(Service::Edsm, t), Duration::ZERO);
        assert_eq!(l.reserve(Service::Edsm, t), Duration::from_secs(1));
        assert_eq!(l.reserve(Service::Edsm, t), Duration::from_secs(2));
        // other services are independent and unlimited here
        assert_eq!(l.reserve(Service::Spansh, t), Duration::ZERO);
        assert_eq!(l.reserve(Service::Spansh, t), Duration::ZERO);
        // once the queue has drained there is no wait
        assert_eq!(l.reserve(Service::Edsm, t + Duration::from_secs(10)), Duration::ZERO);
        assert_eq!(Limiter::default().reserve(Service::Ardent, t), Duration::ZERO);
    }
}
