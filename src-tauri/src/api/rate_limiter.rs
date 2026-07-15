use std::{sync::Arc, time::Duration};

use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};

#[derive(Debug)]
pub struct RateLimiter {
    state: Mutex<RateState>,
    max_tokens: f64,
    refill_per_second: f64,
}

#[derive(Debug)]
struct RateState {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_tokens: u32, window: Duration) -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(RateState {
                tokens: max_tokens as f64,
                last_refill: Instant::now(),
            }),
            max_tokens: max_tokens as f64,
            refill_per_second: max_tokens as f64 / window.as_secs_f64(),
        })
    }

    pub async fn acquire(&self) {
        loop {
            let wait = {
                let mut state = self.state.lock().await;
                let now = Instant::now();
                let elapsed = now.duration_since(state.last_refill).as_secs_f64();
                state.tokens = (state.tokens + elapsed * self.refill_per_second).min(self.max_tokens);
                state.last_refill = now;

                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    None
                } else {
                    let needed = 1.0 - state.tokens;
                    Some(Duration::from_secs_f64(needed / self.refill_per_second))
                }
            };

            match wait {
                Some(duration) => sleep(duration).await,
                None => return,
            }
        }
    }
}
