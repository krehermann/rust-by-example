// println! derives from std::fmt. All types that want to use std::fmt traits (and thus println!, etc)
// require an implementation.

// we can use #[derive] to automatically create the fmt::Debug implementation

// not printable
struct Unprintable(i32);

// `derive` attribute creates implementation to make this struct printable
#[derive(Debug)]
struct DebugPrintable(i32);

// nest struct for illustration
#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {
    let unsayable = Unprintable(3);
    /* won't compile 
    println!("{0:?}", unsayable);
    */

    let debuggable = DebugPrintable(3);
    println!("{debuggable:?}");

    // std library types automatically support debug
    println!("{:?} months in year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "slater", "christian", actor="actor's");

    // nested debug is a bit ugly
    println!("deep debugable looks like {:?}", Deep(debuggable));
    // nested debug is a bit ugly
    println!("deep debugable with #? print-print {:#?}", Deep(DebugPrintable(3)));

    // pretty print a struct with debug attribute
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u32
    }

    let name = "Santa";
    let age = 687;
    let santa = Person{name, age};
    println!{"{santa:#?}"};
}