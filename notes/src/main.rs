use notes;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::ops::{Add, Index};

fn main() {
    // let pair = Pair::new(10, 20);
    //
    // let str_pair = Pair::new("10", "20");
    // str_pair.to_string();
}

pub struct Pair<T> {
    x: T,
    y: T,
    z: i32,
}

/// =============================================================================== ///
/// ======================== NORMAL METHODS IMPLEMENTATIONS======================== ///

/// Normal implementation. These methods are present for Pair<T>
/// for every type T (no bounds on the type).
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y, z: 0 }
    }
    fn x_ref(&self) -> &T {
        &self.x
    }
    fn y_ref(&mut self) -> &T {
        &self.y
    }
}

/// Normal implementation. These methods are present for Pair<T> for
/// every type T that satisfy the traits Display and PartialOrd.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// =============================================================================== ///
/// ======================== SIMPLE TRAITS IMPLEMENTATIONS ======================== ///

/// In general, a single trait can be implemented only once by one specific type.
/// If this was not true, a type could have several implementations for a trait
/// and will not make sense. Note that the 'single trait' definition could be
/// a trait with no generics or a single trait of a generic trait family.

/// ------------------------------------------------------------------------ ///
/// Trait implementation. We can implement a trait on Pair<T> for
/// every possible T. Every Pair<T> will have this implementation.
/// This disallows further implementations for other T's subtypes
/// since the implementation below covers all the possible Pair<T>.
impl<T> Write for Pair<T> {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Ok(0)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// ------------------------------------------------------------------------ ///
/// Bounded trait implementation. We can conditionally implement a trait
/// for any type that implements another trait. These implementations are
/// called 'blanket implementations'. Note that we cannot re-implement a
/// trait if some previous implementations potentially overlaps with the
/// new implementation (even in the future).

impl<T: ToString> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("Point(x: {}, y:{})", self.x.to_string(), self.y.to_string())
    }
}

/// Both the followings implementations are not
/// allowed, since the one above could overlap.
// impl ToString for Pair<Vec<u32>> {
//     fn to_string(&self) -> String {
//         format!(
//             "Point(x: {}, y:{})",
//             self.x.iter().map(&u32::to_string).collect::<String>(),
//             self.y.iter().map(&u32::to_string).collect::<String>(),
//         )
//     }
// }

// impl<T: Display> ToString for Pair<T> {
//     fn to_string(&self) -> String {
//         format!("Point(x: {}, y:{})", self.x.to_string(), self.y.to_string())
//     }
// }

/// ================================================================================ ///
/// ======================== GENERIC TRAITS IMPLEMENTATIONS ======================== ///
/// Generic trait implementations. Each trait derived/specialized form the generic
/// trait is independent and could be implemented. So, a generic trait could be
/// implemented multiple times as long as the implementor type (Pair<T> here)
/// doesn't have overlapping implementations for a trait.

/// ======================================================================== ///
/// Works because we are implementing different traits, since TraitA<i32> and
/// TraitA<f64> are effectively different traits (even if they belong to the
/// same "trait family").
trait TraitA<A> {
    fn trait_a(&self, val: A);
}

impl<T> TraitA<f64> for Pair<T> {
    fn trait_a(&self, val: f64) {}
}

impl<T> TraitA<i32> for Pair<T> {
    fn trait_a(&self, val: i32) {}
}

/// Not allowed, the impl above could overlap,
/// since String is a subtype of the type T.
// impl TraitA<i32> for Pair<String> {
//     fn trait_a(&self, val: i32) {}
// }

/// This works. TraitA<String> implementation
/// doesn't have previous implementations.
impl TraitA<String> for Pair<u8> {
    fn trait_a(&self, val: String) {}
}

/// ======================================================================== ///
trait TraitB<B> {
    fn trait_b(&self, g: B);
}

impl<B> TraitB<B> for Pair<u8> {
    fn trait_b(&self, g: B) {}
}

impl<B> TraitB<B> for Pair<i32> {
    fn trait_b(&self, g: B) {}
}

/// Doesn't work since TraitB<f32> on Pair<i32> was already
/// implemented above (impl<B> TraitB<B> for Pair<i32>).
// impl TraitB<f32> for Pair<i32> {
//     fn trait_b(&self, g: f32) {}
// }

/// This works. TraitA<f32> is implemented on Pair<u8>
/// and Pair<i32> but never on Pair<i64>.
impl TraitB<f32> for Pair<i64> {
    fn trait_b(&self, g: f32) {}
}

/// ======================================================================== ///
trait TraitC<C> {
    fn trait_c(&self, val: C);
}

/// This locks every other implementations. Every trait<C> is already
/// implemented since C is 'all' by all types T that are ToString. The
/// trait ToString could be implemented in the future on several types
/// (including Vec<i32>, and as a consequence to Pair<T>, thanks to the
/// first impl below). This could lead to overlapping implementations.
impl<T, C> TraitC<C> for Pair<T>
where
    T: ToString,
{
    fn trait_c(&self, val: C) {}
}

// impl<C> TraitC<C> for Pair<Vec<i32>> {
//     fn trait_c(&self, val: C) {}
// }

/// ======================================================================== ///
trait TraitSimple {
    fn trait_simple(&self, val: i32);
}

trait TraitD<D> {
    fn trait_d(&self, val: D);
}

/// This works since the bound TraitSimple could be implemented
/// by TestStruct only in the current crate. As a result, nobody
/// else in the future could create an overlapping implementation
/// where T (a TraitSimple) will be the TestStruct type.
impl<T, D> TraitD<D> for Pair<T>
where
    T: TraitSimple,
{
    fn trait_d(&self, val: D) {}
}

struct TestStruct {}

impl<D> TraitD<D> for Pair<TestStruct> {
    fn trait_d(&self, val: D) {}
}
