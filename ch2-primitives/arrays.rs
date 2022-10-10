use std::mem;

// borrow a slice
fn analyze_slice(slc: &[i32]) {
    println!("borrowed slice. first elem: {}", slc[0]);
    println!("borrowed slice. len {}", slc.len());
}

fn main() {
    // array are fixed type and fixed length
    let xs: [i32;5] = [1,2,3,4,5];
    // length and type can be inferred
    let infered_xs = [2,4,6,8];

    println!("int slice len {}, first elem {}", xs.len(), xs[0]);
    println!("infered slice len {}, first elem {}", infered_xs.len(), infered_xs[0]);

    // initialize all elems to the same value
    let ys: [i32;500] = [0;500];
    println!("number of elems in big array {}", ys.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow whole array as slice");
    analyze_slice(&xs);

    // slices point to a section of an array. indices are
    // [)
    println!("borrow 1..4 slice of array");
    analyze_slice(&ys[1..4]);

    // empty slice example `&[]`
    let empty_array: [u32;0] =[];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // use `.get` to safely access elemetns of array. 
    // it returns an `Option` that can be matched
    for i in 0..xs.len()+1 {
        match xs.get(i) {
            Some(xval) => println!("got {}:{}", i, xval),
            None => println!("Slow down! {} is too far", i)
        }
    }
    // direct access beyond allocated size caused runtime panic
    println!("{}",xs[5]);
}