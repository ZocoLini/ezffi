mod string;
#[cfg(feature = "generics")]
mod vec;

pub use string::EzFfiString;
#[cfg(feature = "generics")]
pub use vec::EzFfiVec;
