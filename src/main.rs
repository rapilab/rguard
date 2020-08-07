use crate::rguard::RGuard;

pub mod rguard;

fn main() {
    let guard = RGuard::default();
    guard.execute();
}
