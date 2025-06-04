fn create_number() -> &i32 {
    let x = 5;
    &x // ERROR: `x` doesn’t live long enough!
}