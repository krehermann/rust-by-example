// import crates with `use` 
use std::fmt;

// implement the fmt::Dispaly trait on a struct

struct Displayable(i32);

impl fmt::Display for Displayable {
    // the trait must adhere to function name and signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the first element to stream `f`
        write!(f, "{}", self.0)
    }
}

// fmt:Display is cleaner than Debug at the price of being specific.
// Generic `std` lib types, like Vec<T> do not have Display trait
// new container types that are not generic can implement it

#[derive(Debug)]
struct MinMax(i64,i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

// Define struct where fields can be compared by name
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}


// Point in the complex plane 
#[derive(Debug)]
struct ComplexPoint {
    real: f64,
    img: f64,
}

impl fmt::Display for ComplexPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

fn main() {
    let minmax = MinMax(9,14);
    println!("Display MinMax: {}", minmax);
    println!("Debugg MinMax: {:#?}", minmax);

    let p = Point2D{x:4.1, y:5.7};
    println!("display point2D: {}", p);
    println!("debug Point2D: {:#?}",p);


    let cp = ComplexPoint{real:3.3, img: 7.2};
    println!("display ComplexPoint: {}", cp);
    println!("debug ComplexPoint: {:#?}",cp);
}