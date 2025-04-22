#[unsafe(no_mangle)]
pub extern "C" fn magic_num() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = magic_num();
        assert_eq!(result, 42);
    }
}
