#![no_std]

pub mod coord;
pub mod finder;
pub mod noise;
mod util;
pub mod worldgen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
