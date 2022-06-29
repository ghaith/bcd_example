impl Bcd<String> for u32 {
    fn to_bcd(self) -> String {
        let mut current = self;
        let mut result = String::new();
        while current != 0 {
            let digit = current % 10;
            //Convert to binary representation
            // if result.is_empty() {
            //     result = format!("{digit:04b}");
            // } else {
            //     result = format!("{digit:04b}_{result}");
            // }
            result = format!("{digit:04b}{result}");
            current = current / 10;
        }
        result
    }

}

impl Bcd<u32> for u32 {
    fn to_bcd(self) -> u32 {
        let mut current = self;
        let mut result =  0;
        let mut loc = 0;
        while current != 0 {
            let digit = (current % 10) << loc;
            result = result | digit;
            current = current / 10;
            loc = loc + 4;
        }
        result
    }
}