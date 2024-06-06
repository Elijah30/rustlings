// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}
