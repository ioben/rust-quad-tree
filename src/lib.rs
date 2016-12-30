#![feature(box_patterns)]
#![feature(repeat_str)]

mod tree;
mod primitives;

pub const D: usize = 2;
pub const N: usize = 1 << D; // 2 ^ D

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
