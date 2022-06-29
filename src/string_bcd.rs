use crate::Bcd;

/// convert x to binary string: format!("{x:04b}")
impl Bcd<String> for u32 {
    fn to_bcd(self) -> String {
        todo!("Coming Soon");
    }

}


#[cfg(test)]
mod tests {

    use crate::Bcd;

    #[test]
    fn single_digit() {
        let result : String = 5.to_bcd();
        assert_eq!(result, "0101");
    }

    #[test]
    fn double_digit() {
        let result : String = 12.to_bcd();
        assert_eq!(result, "00010010");
    }

    #[test]
    fn triple_digit() {
        let result : String = 345.to_bcd();
        assert_eq!(result, "001101000101");
    }

}
