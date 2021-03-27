// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($v:expr) => {
        println!("Look at this other macro: {}", $v);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
