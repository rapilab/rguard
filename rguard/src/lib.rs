pub mod configuration;
pub mod obfuscate;
pub mod optimize;
pub mod preverify;
pub mod shrink;

pub mod infra;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
