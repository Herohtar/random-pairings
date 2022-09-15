mod person;
mod pairing_generator;

pub use pairing_generator::PairingGenerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
