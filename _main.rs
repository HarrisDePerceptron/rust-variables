
use std::fmt;
use crate::garden::vegetables::Asparagus;


pub mod garden;

use variables::car;

struct User {
    active: bool,
    name: String,
    email: String,
    count: u64
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.name, self.email, self.count, self.active)
    }
}

#[derive(Debug)]
struct Color (i32, i32, i32);

#[derive(Debug)]
struct Point (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64
}


impl  Rectangle {
    fn area(&self) ->  u64{
        return self.width * self.height;
    }

    fn can_hold(self: &Rectangle, rect: & Rectangle) -> bool{
        self.width >= rect.width && self.height>= rect.height
    }

    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

#[derive(Debug)]
struct Ipv4Addr {
    addr:  (u8,u8,u8,u8),
    mask: (u8,u8,u8,u8)
}

#[derive(Debug)]
struct Ipv6Addr {
    addr: String
}


#[derive(Debug)]
enum IpAddr {
    V4 (Ipv4Addr),
    V6 (Ipv6Addr)
}

enum IpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String)

}

enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor (u8,u8,u8)
}
impl Message {
    fn call(&self){
        println!("Message called()");

    }
}


#[derive(Debug)]
enum Note {
    TEN (u8), 
    HUNDRED(u8),
    THOUSAND(u32),
    FIVETHOUSAND(u32)
}

fn main() {
    println!("hello rust!!");

    let mut s = String::from("hello world");
    {
        let i = first_word(&mut s);

        println!("first word at index {}", i);
    }
    

    println!("first word at index `{}`", s);
    
    let ii = first_word(&mut s);
    let iii = first_word(&mut s);

    println!("first word at index `{}`", iii);


    let l = "hello world";
    let ll = &l;


    let res = first_word(&l);
    println!("the res is {}",res);


    let slice = &s[..6];


    let fw = first_word(slice);

    println!("First world: {}", fw);

    let a = [1,2,3,4,5,6];

    for i in a.iter(){
        println!("arr: {}", i);

    }

    let arr_slice = &a[..2];
    assert_eq!(arr_slice, &[1,2]);


    for i in 1..10 {
        println!("i: {}", i);

    }


    let user = User {
        active: true,
        email: String::from("harris@gmail.com"),
        count: 1,
        name: String::from("harris")
    };

    println!("user : {:?}", user);
    
    let t= (&user, &user);

    let u1 = build_user("hey@gmail.com", "hey123");
    
    println!("Build user: {:?}", u1);
    let u3 = User {email: String::from("user2@gmail.com"), ..u1};
    println!("user 2 copy constructor: {:?}", u3);



    let black = Color(12,13,14);


    println!("black {:?}", black);


    let rec = Rectangle {
        height: 10,
        width: 5
    };
    println!("the rectangle area {}", rec.area());


    let rec2 =  Rectangle {
        height: 9,
        width: 4
    };
    

    let rec3 = Rectangle {
        height: 11,
        width: 4
    };


    println!("rec can hold rec2?: {}", rec.can_hold(&rec2));
    println!("rec can hold rec2?: {}", rec.can_hold(&rec3));


    let sq = Rectangle::square(4);
    
    println!("Square is: {:#?}", sq);


    let home = IpAddrKind::V4(127, 0, 0, 1);
    let homev6 = IpAddrKind::V6(String::from("::1"));


    let homev4 = IpAddr::V4(Ipv4Addr { addr: (127,0,0,1), mask: (255,255,255,0) });

    
    println!("home ip4 addr : {:?}", homev4 );

    let message = Message::Write(String::from("hello world"));
    
    let moveMessage = Message::Move { x: 50, y: 60 };
    
    let v = Some("val");
    let nv : Option<&str>= None;
    
    if nv.is_none() {
        println!("nv is none");

    }

    message.call();


    let note = Note::HUNDRED(5);
    println!("Note value is {}", calculate_value(note));

    
    let po = plus_one(Some(10));
    println!("plus one: {}", po.expect("plus one failed"));

    if let Some(v) = po {
        println!("the value of plus one is {}", v);

    }


    let nn =  plus_one(None);
    
    if let Some(nn) = nn {
        println!("should not print this");
    }



    let dice: Option<u8> = Some(10);
    let rr: Option<u8> = match dice {
        Some(4) => None,
        _ => None
    };


    let m = car::make::CarMake::HONDA(String::from("HONDA"));
    

    if let car::make::CarMake::HONDA(val) =  m {
        println!("the car make is honda: {}", val);

    } 

    




}


fn plus_one(v: Option<u8>) -> Option<u8>{
    match v {
        Some(v) => Some(v+1),
        None => None
    }
}

fn calculate_value(note: Note)-> u32{
        match note {
            Note::TEN(v) => u32::from(v)*10,
            Note::HUNDRED(v) =>  u32::from(v) * 100,
            Note::THOUSAND(v) => u32::from(v) * 1000,
            Note::FIVETHOUSAND(v) => u32::from(v) *  5000,
            
        }
}


fn first_word(s: & str ) -> & str {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
                return & s[..i];
        }
    }

    return & s[..];

}


fn build_user (email: &str, username: &str) -> User{
    User {
        email: String::from(email),
        name: String::from(username),
        active: true,
        count: 1
    }
}