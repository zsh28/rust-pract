fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat; // This destructures the tuple into `name` and `age`.

    println!("{name} is {age} years old");
}
