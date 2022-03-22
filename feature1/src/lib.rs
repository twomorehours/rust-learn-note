#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cfg(feature = "abc")]
pub mod abc {
    pub fn calc(a: i32, b: i32) -> i32 {
        a + b
    }
}
