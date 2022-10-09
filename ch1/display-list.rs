// chaining and intro to error propogartion
// we want to implement Display for a complex type with
// sequential element, i.e. a list
// the trait defn require a return type of fmt::Result
// however, write! also returns a fmt::Result
// this begs the question, how to compose a fmt::Result
// from prior instances.

// the answer is to chain using the `?` operator

// The question mark (?) operator in Rust is used as an error propagation 
// alternative to functions that return Result or Option types. 
// The `?` operator is a shortcut as it reduces the amount of code
// needed to immediately return Err or None from the types Result<T, Err> 
// or Option in a function.



use std::fmt;

fn main() {
    println!("writing a list of values");
    let l= List(vec![1,2,3]);
    println!("{l}");
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"this uses trailing ?\n")?;
       // write!(f,"this uses trailing ?:")
       let vec = &self.0;
       for (cnt, val) in vec.iter().enumerate() {
        if cnt != 0 {write!(f,", ")?;}
        write!(f,"{cnt}:{val}")?;
       }
        write!(f,"\ndone")
    }
}

