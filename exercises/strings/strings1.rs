// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    solution1();
    solution2();
}

fn solution1() {
    let answer = current_favorite_color1();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color1() -> String {
    String::from("blue")
}

fn solution2() {
    let answer = current_favorite_color2();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color2() -> String {
    (*"blue").to_string()
}
