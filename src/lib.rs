pub use self::domain::*;
pub use self::settings::*;

pub mod dao;
mod domain;
mod settings;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
