// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


/*
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
 */


 mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn invoke_my_macro() {
        my_macro!();
    }
}

fn main() {
    macros::invoke_my_macro(); // Now you can use the module name to access the macro
}
