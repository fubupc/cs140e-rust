//! Timer for user to implements

use core::time::Duration;

pub trait Timer {
    /// Wait for the condition to become true within a specified time duration. If the condition becomes
    /// true before the `timeout` expires, return the elapsed time, otherwise, return an error.
    fn wait_for<C: Fn() -> bool>(&self, condition: C, timeout: Duration) -> Result<Duration, ()>;

    /// Wait for the specified time duration.
    fn wait(&self, d: Duration);

    /// Wait for some CPU cycles.
    fn wait_cycles(&self, n: u64);
}
