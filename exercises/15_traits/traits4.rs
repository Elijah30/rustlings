// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn compare_license_types<T, U>(software: T, software_two: U) -> bool
where
    T: Licensed,
    U: Licensed,
{
    software.licensing_info() == software_two.licensing_info()
}
