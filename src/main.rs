use crate::rguard::RGuard;

pub mod rguard;
pub mod configuration;

fn main() {
    let guard = RGuard::default();
    guard.execute();
}
