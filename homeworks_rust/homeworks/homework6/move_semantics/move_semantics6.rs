// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

fn main() {
    let data = "Rust is great!".to_string();
    let last_char = get_char(&data);
    println!("Last char: {}", last_char);
    string_uppercase(&data);
}

fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(data: &String) {
    let uppercase_data = data.to_uppercase();
    println!("{}", uppercase_data);
}
