// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise,
// even after you already figured it out. If you got everything working and
// feel ready for the next exercise, remove the `I AM NOT DONE` comment below.


fn type_of<T>(_: &T) {
    println!("type: {}", std::any::type_name::<T>());
}

fn main() {
    let x: u64 = 3;
    println!("x has the value {}", x);
    type_of(&x);

    let x = 5;
    println!("x has the value {}", x);
    type_of(&x);

    let mut y = 3;
    println!("y has the value {}", y);
    type_of(&y);

    y = 5;
    println!("y has the value {}", y);
    type_of(&y);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is an u32 constant with value {}", MAX_POINTS);
}
