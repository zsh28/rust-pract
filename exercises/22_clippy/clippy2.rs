fn main() {
    let mut res = 42;
    if let Some(x) = Some(12) {
        // TODO: Fix the Clippy lint.
        res += x;
    }

    println!("{res}");
}
