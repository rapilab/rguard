use rguard_core::shrink::Shrinker;

fn main() {
    execute();
}

fn execute() {
    shrink();
    optimize();
    obfuscate();
    preverify();
    write_output();
}

fn shrink() {
    let shrinker = Shrinker::default();
    shrinker.execute();
}

fn optimize() {}

fn obfuscate() {}

fn preverify() {}

fn write_output() {}
