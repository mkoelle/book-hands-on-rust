#![warn(clippy::all, clippy::pedantic)]
fn main() {
    let my_list = ["ONE", "TWO", "THREE"];
    for item in &my_list {
        println!("{item}");
    }
}
