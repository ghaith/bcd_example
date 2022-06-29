mod u32_bcd;
mod string_bcd;

pub trait Bcd<T> {
    fn to_bcd(self) -> T;
}


