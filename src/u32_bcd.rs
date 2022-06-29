use crate::Bcd;


impl Bcd<u32> for u32 {
    fn to_bcd(self) -> u32 {
        todo!("Comming Soon");
    }
}


#[cfg(test)]
mod tests {

    use crate::Bcd;

    #[test]
    fn single_digit() {
        let result : u32 = 5.to_bcd();
        assert_eq!(result, 0b0101);
    }

    #[test]
    fn double_digit() {
        let result : u32 = 12.to_bcd();
        assert_eq!(result, 0b0001_0010);
    }

    #[test]
    fn triple_digit() {
        let result : u32 = 345.to_bcd();
        assert_eq!(result, 0b0011_0100_0101);
    }

}