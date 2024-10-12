// #![no_std]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod bitline;
pub mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
