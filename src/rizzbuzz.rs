use num_traits::{identities::Zero, PrimInt};
use std::fmt;

#[derive(Debug, PartialEq)]
enum RizzbuzzOutputs<T> {
    Rizz,
    Buzz,
    RizzBuzz,
    Num(T),
}

impl<T> fmt::Display for RizzbuzzOutputs<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RizzbuzzOutputs::Rizz => write!(f, "Rizz"),
            RizzbuzzOutputs::Buzz => write!(f, "Buzz"),
            RizzbuzzOutputs::RizzBuzz => write!(f, "RizzBuzz"),
            RizzbuzzOutputs::Num(ref val) => write!(f, "{}", val),
        }
    }
}

fn fizzbuzz<T>(num: T) -> RizzbuzzOutputs<T>
where
    T: PrimInt + Zero,
    T: Copy + Clone,
{
    let zero = T::zero();
    let three = T::from(3).expect("Could not convert '3' to generic type");
    let five = T::from(5).expect("Could not convert '5' to generic type");

    match (num % three, num % five) {
        (x, y) if x == zero && y == zero => RizzbuzzOutputs::RizzBuzz,
        (x, _) if x == zero => RizzbuzzOutputs::Rizz,
        (_, x) if x == zero => RizzbuzzOutputs::Buzz,
        _ => RizzbuzzOutputs::Num(num),
    }
}

// TODO: take in max range to iterate to
pub fn run() {
    for x in 1..=100 {
        println!("{}", fizzbuzz(x))
    }
}
