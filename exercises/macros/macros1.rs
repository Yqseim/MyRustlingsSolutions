// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// I AM DONE

//#[macro_export] <- unecessary in this case
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}
