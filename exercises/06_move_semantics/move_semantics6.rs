// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone()); // Cloner data car get_char prend la propriété de data
    println!("Last char: {}", last_char);

    let uppercased = string_uppercase(data); // Passer la propriété de data à string_uppercase
    println!("{}", uppercased);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) -> String {
    data.to_uppercase() // Retourner une nouvelle chaîne en majuscules
}

    data = &data.to_uppercase();

    println!("{}", data);
}
