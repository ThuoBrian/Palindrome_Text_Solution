fn main() {
    println!("Enter a word:");

    let mut word = String::new();

    std::io::stdin()
        .read_line(&mut word)
        .expect("Fail to read the entered word");

    is_palindrome_text(&word.trim());
}

fn is_palindrome_text(text: &str) {
    let x_transform = |x: &str| -> String { x.to_lowercase().chars().rev().collect() };
    if x_transform(text) == text.to_lowercase() {
        println!("Yes, the word is a palindrime");
    } else {
        println!("No, the word is a palindrome");
    }
}
