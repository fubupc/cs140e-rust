// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.to_ms() == other.to_ms()
    }
}

impl Duration {
    fn to_ms(&self) -> u64 {
        match self {
            &Duration::MilliSeconds(x) => x,
            &Duration::Seconds(x) => x as u64*1000,
            &Duration::Minutes(x) => x as u64*1000*60,
        }
    }
}

use Duration::{Seconds, Minutes, MilliSeconds};

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
