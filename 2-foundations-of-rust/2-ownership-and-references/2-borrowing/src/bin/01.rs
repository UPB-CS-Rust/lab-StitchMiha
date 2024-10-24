//! Make me compile only by reordering the lines in `main()`, but without
//! adding, changing or removing any of them.

fn main() {
    let mut x = 100;
    let y = &mut x; // both  y and z are mutable references to x
    *y += 100; // we dropped it here the reference before using it again to create z
    let z = &mut x;
// we can only have one mutable reference to a variable
    *z += 1000;
    assert_eq!(x, 1200);
}
