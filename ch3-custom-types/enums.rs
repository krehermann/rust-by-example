// `enum` to classify a web event. Both the name and type
// together specify a variant
enum WebEvent {
    // `unit` like
    PageLoad,
    PageUnload,
    // tuple like
    KeyPress(char),
    Paste(String),
    // and c-struct like are all valid variants
    Click {x: i64, y: i64},
}

// handle each WebEvent variant
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // destructure tuple entry inside the enum
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted {}",s),
        // destructure elems of c-struct
        WebEvent::Click{x,y} => println!("clicked ({},{})",x,y),
    }
}

// enums support type aliasing, used for compacting code
enum VeryVerboseEnumOfStuffToDoWithNumbers {
    Add,
    Subtract
}
type Operations = VeryVerboseEnumOfStuffToDoWithNumbers;

// type aliasing is often seen in practice in the `impl` blocks using Self alias
impl VeryVerboseEnumOfStuffToDoWithNumbers {
    fn run(&self, x:i32, y:i32) -> i32 {
        match self {
            Self::Add => x +y,
            Self::Subtract => x-y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("important text".to_owned());
    let click = WebEvent::Click {x:20, y: 20};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let plus = Operations::Add;
    println!("1 + 2 = {}", plus.run(1,2))
}
