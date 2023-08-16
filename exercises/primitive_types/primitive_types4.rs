// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // then ending_index is +1 past the slice of where you want
    // internally rust gets the slice by startIndex..(endIndex-startIndex)
    // hence 4-1 = 3, then length of [2,3,4]
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
