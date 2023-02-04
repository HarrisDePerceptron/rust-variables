

use std::fmt::Display;
use std::{thread, num};
use std::time::Duration;


#[derive(Debug, Clone, Copy, PartialEq)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>

}


impl  Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{

        user_preference.unwrap_or_else(||{
            self.most_stocked()
        })

    }

    fn most_stocked(&self)-> ShirtColor{

        let mut num_red = 0;
        let mut num_blue = 0;
        
        for sh in &self.shirts{
            match  sh {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue +=1
            }
        }

        if num_red > num_blue {
            return ShirtColor::Red
        }else {
            return ShirtColor::Blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}


#[derive(Debug)]
struct Shoe {
    style: String, 
    size: i32
}

fn main() {
    println!("hello world ");

    let inv = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red]
    };


    println!("Give away: {:?}", inv.giveaway(None));
    let s = Some(ShirtColor::Blue);

    println!("hey giveaway: {:?}",inv.giveaway(s));


    let extensive_closure = |num: i32| ->i32 {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(1));
        num
    };


    println!("extensive closure {}, ",extensive_closure(100));


    let list = vec![1,10,20,30,100,500];
    println!("Before defining clousure {:?}", list);
    let only_borrows = ||{
        println!("From closure {:?}", list);
    };
    println!("After defining clousure {:?}", list);


    only_borrows();
    println!("After calling borrows {:?}", list);



    let mut list2 = vec![1,100,200,1000];
     println!("List 2 is: {:?}", list2);


    let mut mutable_borrow = || list2.push(2000);
    
    mutable_borrow();
    
    println!("read borrow of list2: {:?}", list2);
    

    let t = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("Inside a thread non mut borrow: {:?}", list2);
    }).join().unwrap();

    // println!("After calling thread: {:?}",  list2);



    let mut rectangles = vec![
        Rectangle {width: 30, height: 20},
        Rectangle {width: 20, height: 20},
        Rectangle {width: 10, height: 20}
    ];


    rectangles.sort_by_key(|r| r.width );

    

    println!("Sorted rectangles: {:?}", rectangles);


    let mut sort_operation:Vec<String> = vec![];
    
    let value = String::from("value");

    let mut num_sort_operations: i32 = 0;
    rectangles.sort_by_key(|r| {
        println!("hello world inside");
        num_sort_operations += 1;
        r.width
    });


    println!("Num sort operations: {}", num_sort_operations);


    let v = vec![1,2,3,4,5];
    let iter = v.iter();

    for i in iter {
        println!("iter: {}", i);
    }
    

    let v1 = vec![1,2,3];
    
    let mut iitr = v1.iter();

    let n = iitr.next();

    assert_eq!(iitr.next(), Some(&2));
    assert_eq!(iitr.next(), Some(&3));


    let v2: Vec<i32> = v1.iter().map(|x| x*2).collect();
    for i in v2.iter() {
        println!("Collect value: {}", i);
    }


    assert_eq!(v2, vec![2,4,6]);



    let shoes = vec![
        Shoe{size: 10, style: String::from("st1")},
        Shoe{size: 11, style: String::from("st1")},
        Shoe{size: 12, style: String::from("st1")},
        Shoe{size: 10, style: String::from("st2")},
        Shoe{size: 11, style: String::from("st2")},
        Shoe{size: 12, style: String::from("st3")}
        ];





}


fn shoes_in_size(shoes: Vec<Shoe>,sz: i32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == sz).collect()


}


#[cfg(test)]
mod TestShoes {
    use super::*;


    #[test]
    fn test_shoes() {
        let shoes: Vec<Shoe> = vec![
            Shoe{size: 10, style: String::from("st1")},
            Shoe{size: 11, style: String::from("st1")},
            Shoe{size: 12, style: String::from("st1")},
            Shoe{size: 10, style: String::from("st2")},
            Shoe{size: 11, style: String::from("st2")},
            Shoe{size: 12, style: String::from("st3")}
            ];


        let result = shoes_in_size(shoes, 10);
        println!("Vector shoes size 10 : {:#?}", result);
        assert!(result.len()==2);

    }
}