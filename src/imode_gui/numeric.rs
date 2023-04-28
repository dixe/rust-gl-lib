pub trait Numeric : Clone + Copy + PartialEq + PartialOrd + 'static {

    fn to_f64(self) -> f64 ;

    fn from_f64(num: f64) -> Self;
}

macro_rules! impl_numeric {
    ($t: ident) => {
        impl Numeric for $t {
            fn to_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(num: f64) -> Self {
                num as Self
            }
        }
    }
}


impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);


impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);

impl_numeric!(f32);
impl_numeric!(f64);
