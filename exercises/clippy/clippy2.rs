// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    if let Some(option) = Some(12) {
        res += option;
    }
    println!("{}", res);
}
