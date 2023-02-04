

use std::fmt::Display;



#[derive(Debug)]
struct SomeStruct<'a> {
    a: &'a i32,
    b: &'a i32
    
}

fn main() {
    println!("hello world ");

    let mut r;


    {
        
        let x  = String::from("hello");
        r = &x;

        println!("R ref is {}", r);

    }
    let a = String::from("hey hey");
    r = &a;


    println!("R is {}", r);


    let a = 14;
    let b = 13;
    
    let lon = longest(&a, &b);
    println!("Longest  number is {}", lon);

    {
        let c = 200;
        let l = longest(& c, &c);
        println!("longest inner is {}", l);

    }

    let d = 100;
    let s;

    {

        let c = 200;

        s = SomeStruct {
            a: &d,
            b: &d
        };


        println!("Some struct : \n {:?}", s);
        
    }


    println!("Some struct : \n {:?}", s);

    let st: &'static str = "hiya. whats up  mate";
    
    let st2: &str = "hello guys";

    let var = 10;
    let var2 = 20;
    let res;


    {
        let var3 = 500;
        res = longest2(&var, &var3);
        println!("the res is {}", res);

    }


    longest_With_annoucement("hello", "world", "thisis an annoucement");



}


fn longest<'a>(x: &'a i32, y: &i32)->  &'a i32{
    if x > y {
        x
    }else{
       let d = &500;
       d

    }
}


fn longest2<'a> (x: &'a i32, y: &'a i32) -> &'a i32 {
    if x> y{
        x
    }else{
        y
    }
}


fn longest_With_annoucement<'a, T> (x: &'a str, y: &'a str, ann: T)
-> &'a str
where 
T: Display
{

    println!("Announcement: {}",  ann);

    if x> y{
        x
    }else{
        y
    }

    
}