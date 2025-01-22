use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

// Add<Meters> sets the value of Rhs type parameter instead of using default of Self
impl Add<Meters> for Millimeters    
{
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters
    {

        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Associated types
impl Iterator for Counter
{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>
    {

    }
}
// Generic
pub trait Iterator<T>
{
    fn next(&mut self) -> Option<T>;
}

// Generic: must annotate types in each implementation
// since we can also implement Iterator<String> for Counter or any other type
// We could have multiple implementation of Iterator for Counter.
// Trait with generic parameter = can be implemented for a type multiple times

// Associated types: no need to annotate types. cannot implement trait on a type multiple times
// can only choose what the type of Item will be once, since there can be only one impl Iterator for Counter

// When using generic type parameter, can specify default concrete type for generic type.
// eliminates needs for implementors of trait to speicfy a concrete type if default type works
// example:operator overloading