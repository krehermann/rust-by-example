// examples of printing formatted text
fn main() {
    // {} is substituted with arguments
    println!("{} curly sub", "this is");

    // substitution can be indexed
    println!("{0} is zeroth arg, {1} is first, which is positioned after {0}", "dog", "cat");

    // named arguments
    println!("{subject} {verb} {object}", object="el perro perezoso", subject= "el gato rapido", verb="come");

    // formatting specified by :<char>
    println!{"Base 10: {}", 247};
    println!{"Base 2: {:b}", 247};
    println!{"Base 8: {:o}", 247};
    println!{"Base 16: {:x}", 247};
    println!{"Base 16, capital: {:X}", 247};
    
    // alignment
    // right align with 4 leading spaces
    println!("{arg:>5}", arg =1);
    // pad with 0
    println!("{arg:0>5}", arg=1);
    // alignment with named arg using trailing $
    println!{"align by {width} {arg:width$}", arg=1, width=5}

    // rust checks that all args are used
    /* won't compile 
    println!("the name is {0}, {1} {0)", "Bond")
    */
    println!("the name is {0}, {1} {0}", "Bond", "jimmy");

    // in rust >= 1.58, directly capture variables as args
    let number = 1.0;
    let width = 5;
    println!("print with vars {number:0>width$}");

    // exercise print pi, 3.141592 to 2 decimal places
    let pi= 3.141592;
    println!("Pi={pi} is roughly {input:.2}", input=pi);

}