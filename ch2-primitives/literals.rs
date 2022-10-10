fn main() {
    // type annotation on a boolean
    let logical: bool =true;
    // type annotation is optional
    let b = false;
    // and types can be inferred
    let inferred_bool = b && true;

    // integers can be annotated in normal convention,
    // with suffix, or by default when explicit annotation
    // is omitted
    let a_float: f64 = 3.0; // regular annotation
    let an_int = 5i32; // suffix for i32 inte
    let default_float = 3.9; // `f64`
    let defualt_int = 7; // `i32`

    // mutable variable's value can change
    let mut inferred_type = 12;
    inferred_type=93;

   /*  // the type cannot change
    inferred_type = true;
    */
    // variable shadowing is allowed
    let inferred_type = 4.7;

    // operators
    println!("1+2={}",1u32+2);
    println!("1-2={}", 1i32-2);
    //println!("subtraction of unsigned int doesn't compile: 1-2={}", 1u32-2);
    
    // boolean logic
    println!("true AND false {}", true && false);
    println!("true OR false {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    println!("0011 AND 0101: {:04b}", 0b0011u32& 0b0101u32);
    println!("0011 OR 0101: {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101: {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 {:04b}, {}", 1u32 << 5, 1 << 5);
    println!("0x80>>2: 0x{:x}", 0x80u32 >> 2);

    // underscores in long numeric literals improve readbility
    println!("one MILLION dollars {}",1_000_000u32);
}