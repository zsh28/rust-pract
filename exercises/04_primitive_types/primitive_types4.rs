fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // The slice should contain the elements 2, 3, and 4.
        let nice_slice = &a[1..4]; // This creates a slice from index 1 to index 4 (exclusive).

        assert_eq!([2, 3, 4], nice_slice);
    }
}
