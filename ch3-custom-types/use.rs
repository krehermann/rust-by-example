enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // use `use` declaration to simplify scoping
    use crate::Status::{Poor, Rich};
    // automatically `use` each name inside `Work`
    use crate::Work::*;

    let status = Poor; // eq Status::Poor
    let work = Civilian;

    match status {
        Rich => println!("money money money!"),
        Poor => println!("no monies"),
    }

    match work {
        // no scoping b/c of `use`
        Civilian => println!("just a person"),
        Soldier => println!("sir yes sir"),
    }
}