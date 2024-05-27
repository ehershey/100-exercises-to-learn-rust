// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// impl From for WrappingU32 {}
// impl Into<WrappingU32> for u32 {
// fn into(self) -> WrappingU32 {
// // let x = WrappingU32 { value: self }
// let x = WrappingU32 { value: self };
// x
// }
// // fn intoWrappingU32<T>
// }

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value: value }
    }
}

// impl<T> Into for u32 {
// fn from(value: u32) -> WrappingU32 {
// WrappingU32(value);

// let new_value:WrappingU32 = new WrappingU32(original_value)
// }
// }
// pub trait Into<T>: Sized {
// fn into(self) -> T;
// }

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
