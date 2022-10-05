// examples of printing formatted text
fn main() {
    // {} is substituted with arguments
    println!("{} curly sub", "this is");

    // substitution can be indexed
    println!("{0} is zeroth arg, {1} is first, which is positioned after {0}", "dog", "cat");

    // named arguments
    println!("{subject} {verb} {object}", object="el perro perezoso", subject= "el gato rapido", verb="come");

}