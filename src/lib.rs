//! # Variable Crate
//! 
//! `variable` crate is just the crate for testing my rust skills
//! this crate contains every example from the rust book
//! 


pub mod car;


mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn print_and_return_value(a: i32) ->  i32{
    println!("hello got value: {}", a);
    a
}



pub mod aggregator;

pub mod testing;

pub mod utils;