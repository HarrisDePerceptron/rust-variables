use std::fs;

fn main() {
    println!("hello rust!!");

    let s = String::from("hello");
    let u = s.to_uppercase();

    println!("the upper case for `{}` is `{}`", s, u);

    let mut ss = String::from("hey");
    ss.push_str(" how");
    ss.push_str(" are");
    ss.push_str(" you");

    println!("Final string is {}", ss);

    let l = "hello guys";

    println!("mutable literal: {}", l);

    // let args: Vec<String> = std::env::args().collect();

    // if args.len() <2 {
    //     panic!("Please provide the file path to proceeed\n");

    // }
    // let path = &args[1];

    // // let content = fs::read_to_string()
    // println!("Path=`{}`", path);

    let x = 5;
    let y = x;

    println!("x={}, y={}", x, y);

    let s1 = String::from("hello");
    let s2 = &s1;


    let s3 = gives_ownership();
    // takes_ownership(s3);
    
    let s4 =  takes_and_return_ownership(s3);

    let mut s5 = String::from("hi. ");
    change(&mut s5);


    {

        let r1 = &mut s5;
    }

    let r1 = &s5;
    let r3 = &s5;
    
    println!("refs: {},{}",r1, r3);
    
    let r2 = &mut s5;


    println!("mutable mut ref: {}",r2);


    println!("s1={}, s1={}, s3={}, s5={}", s1, s2, s4, s5);


    let ref_to_dangle = dangle();
}



fn gives_ownership() -> String{
    let s= String::from("giving ownership");
    return s;

}

fn takes_ownership(s: String){
    println!("ownership s: {}", s);

}

fn takes_and_return_ownership(s: String) -> String {
    s
}


fn change (s: &mut String){
    s.push_str("hey you");

}


fn dangle () -> &String {
    let s= String::from("dangle");
    
    &s
}


fn first_word(s: &String ) -> usize {
    let byte = s.as_bytes();
    
    let index: usize = 0;

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
                index = i;
                break;
        }
    }

    index
}