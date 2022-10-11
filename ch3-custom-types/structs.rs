// hide compiler warnings
#![allow(dead_code)]

// C struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct -- field-less, used in generics
struct Unit;

// tuple struct -- named tuple more or less
struct Pair(i32,f32);

// c struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// struct can be reused/embedded
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Point{x: x_left, y: y_left} = rect.top_left;
    let Point{x: x_right, y: y_right} = rect.bottom_right;

    ((x_left - x_right) * (y_left - y_right)).abs()
}

fn square(point: Point, width: f32) -> Rectangle {
    let Point{x: x_right, y: y_right} = point;
    Rectangle{top_left: point, 
        bottom_right: Point{
        x: x_right + width, y: y_right +width
    }}
}
fn main() {
    let name = String::from("Jim");
    let age =82;
    let jim = Person{name, age};
    println!("jim is {:?}", jim);

    // instantiate a point
    let pnt = Point{x:11.1, y:0.7};
    println!("pnt coords: ({}, {})", pnt.x, pnt.y);

    // use struct update syntax
    let bottom_right = Point{ x: 17.7, ..pnt};
    // y coord is the same value as that from which
    // it was instantiated
    assert_eq!(pnt.y, bottom_right.y);
    println!("new point from first: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure. declare and assign vals to new 
    // variables, left_edge, top_edge by inspecting
    // the contents of an instance of Point 
    let Point { x: left_edge, y: top_edge} = pnt;
    dbg!(top_edge, left_edge); 
    println!("is pnt still around? {:?}",pnt);
    assert_eq!(left_edge, pnt.x);
    assert_eq!(top_edge,pnt.y);

    let rect1 = Rectangle {
        top_left: Point{x: left_edge, y:top_edge},
        bottom_right: bottom_right,
    };
    println!("rect: {:?}", rect1);
/* 
    let rect2 = Rectangle {
        top_left: Point{x: left_edge, y:top_edge},
        bottom_right: bottom_right,
    };
    println!("rect2: {:?}", rect2);
    */

    // destructure a tuple struct
    let pair = Pair(1,0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?}, {:?}", integer, decimal);

    println!("{}",rect_area(rect1));

    println!("square from -> {:?}", square(Point{x:1.0,y:1.0}, 2.0));
}