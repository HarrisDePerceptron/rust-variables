use std::io::Write;
use std::io;



const SECONDS_IN_A_DAY: u32 = 1 * 24 * 60 * 60;

const SECONDS_IN_A_WEEK: u32 = SECONDS_IN_A_DAY * 7;

const SECONDS_IN_A_MONTH: u32 = SECONDS_IN_A_DAY * 30;



fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("value of x is {}", x);
    x = 10;
    println!("value of x is {}", x);
    println!("hey");


    println!("seconds in a day: {}", SECONDS_IN_A_DAY);
    println!("seconds in a week: {}", SECONDS_IN_A_WEEK);
    println!("seconds in a month: {}", SECONDS_IN_A_MONTH);

    let x = 20;
    
    {
        let x = x * 2;
        println!("value of x inside scope: {}", x);

    }

    println!("value of x outside of scope: {}", x);


    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces len: {}", spaces);

    let guess: _ = "42".parse::<u32>().expect("failed to convert");
    println!("Guess is: {}", guess);


    let b:u64 =0b1000000000000000000000000000000000000000000000000000000000000000;
    let ui8: u8 = 255;

    println!("Binary is equal to {}", b);
    println!("unsigned integer val {}", ui8);

    let a = 5;
    let b = 5.5;

    let c = -5;

    let sum = 10 +5;
    let sum2 = 5.5 + 10.5;
    let diff = 10 -5;
    let diff2 = 10.5 - 5.5;

    let prod = 10*5;
    let prod2 = 10.5* 0.5;

    let div = 10/5;
    let div2 = 10.5/5.0;

    let div3: f32 = 10.5 / 2.5;

    let floored = 2/3;

    let remainder = 10 % 5;
    let remainder2 = 10.5 % 5.5;
    
    println!("Remainder is {}.\nRemainder2 is {}", remainder, remainder2);


    let ch = 'z';
    let heart_eyed_cat = '😻';
    let z: char = 'ℤ'; 

    let row = 10;

    for i in 0..10 {
        println!("{}", i);
    }

    for i in 0..100 {
        
        print!("{}", heart_eyed_cat);
        std::io::stdout().flush().expect("enable to flush");
        let current_col = (i+1)% row;

        if current_col== 0 {
            println!("");
            
        }

        
    }

    println!("");


    let tup = (10, 5.5);
    
    let (aa, bb) = tup;


    println!("aa is {} and bb is {}", aa, bb);

    let arr = [1,2,3,4,5];
    let _arr2: [i32; 5]= [1,2,3,4,5];

    let arr3 = [3; 100];

    println!("arary with 5 elems {:?}", arr3);

    let first = &arr[0];
    let second = &arr[1];
    
    println!("first: {}, second: {}", first, second);
    
    let mut index = String::new();
    

    print!("enter index: ");
    io::stdout().flush()
    .expect("unale to flush");

    io::stdin()
    .read_line(&mut index)
    .expect("unable to rea index");

    let index: usize = index
        .trim()
        .parse()
        .expect("unable to parse index");
    

    println!("index is {}", index);
    let selected = &arr[index];
    
    println!("Selected element is {}", selected);

    another_fun(10);

    labeled_measurement(11,'c');

    let x = {
        let y = 10;
        y +1
    };

    println!("Expression x is {}", x);

    let f = five();
    println!("five()={}", f);

    println!("{} plus one is {}", 5,plus_one(5));


    let number = 7;



    if number > 5 {
        println!("expression is true");

    }else{
        println!("expression is false")
    }


    let num = 6;
    if num % 4 == 0 {
        println!("divisible by 4");
    } else if num % 3 == 0 {
        println!("divisible by 3");
    }else if num % 2 == 0{
        println!("divisible by 2");
    }else {
        println!("not divisible by 4,3,2");
    }

    let res = if num==6 {true} else {false};
    
    let mut i = 0;

    loop {
        i+=1;
        
        println!("again");
        if (i+1) % 5 == 0 {
            break;
        }
    }


    let mut numm = 4;
    while numm != 0{
        numm-=1;
        println!("num: {}", numm);

        
    }

    for i in (1..10).rev(){
        println!("i: {}", i);
        
    }
    
}



fn another_fun(x: i32){
    println!("Another function called: {}", x);

}

fn labeled_measurement(x: i32, unit_label: char){
    println!("Labeled measurement {}{}", x, unit_label);

}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}



