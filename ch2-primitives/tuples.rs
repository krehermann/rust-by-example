// tuples is a collection of values of different types
// (T1, T2,...,TN)
// functions can use tuples to return multiple values

use std::fmt;

// tuples as args and return values
fn reverse(pair:(i32,bool)) -> (bool,i32) {
    let (i,b) = pair;
    (b,i)
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n({} {})", self.0, self.1);
        write!(f, "\n({} {})", self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0,m.2,m.1,m.3)
}
fn main() {
    let long_tuple = (1u32, 2u16, 3u32,4u64,
        -1i8,-2i16,
        0.1f32,0.2f64,
        'a',
        true);

    // extract values by index
    println!("first value: {}", long_tuple.0);
    println!("second value: {}", long_tuple.1);

    // tuples can be nested
    let tuple_of_tuples = ((1u8,2u16,2u32),(4u32,-1i8),-2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    /* 
    // long tuples, more than 12 elems, cannot be printed
    let too_long = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    println!("too long, compliation error: cannot be formatted using `{:?}` because it doesn't implement `Debug` {:?}", too_long);
*/

    let pair = (1,true);
    println!("pair is {:?}", pair);
    println!("reversed {:?}", reverse(pair));

    // commas required to distinction tuple with one elem
    // from literal surrounded by parens
    println!("this is one elem tuple `(1,)`  {:?}",(1,));
    println!("this is one literal with parens `(1)` {:?}",(1));

    // tuples can be bound by elements
    let t = (1,"hello", 4.5, true);
    let (a,b,c,d) = t;
    println!("bound values {a}, {b}, {c}, {d}");

    // exercises
    let m = Matrix(1.,2.,3.,4.);
    println!("matrix debug: {:?}", m);
    println!("matrix display: {}",m);

    println!("matrix transpose: {}", transpose(m));
}