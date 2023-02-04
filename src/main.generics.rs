use std::io::Error;



fn main() {
    println!("hello world ");
    let number_list = vec![1,2,3,100,4,5];
    let l1 = largest(&number_list);

    println!("Largest number list:{l1}");


    let v2 = vec![1,2,100,200,40,50];
    let l = largest(&v2);

    println!("Largest v2: {}", l);

    let v3 = vec!['a', 'b', 'c','A', 'C'];

    let l3 = largest(&v3);
    println!("Largest char: {l3}");
    
    
    
}

fn largest<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {
    let mut num = &arr[0];
    for i in arr {
        if i > num {
            num = i;
        }
    }

    return num;    
}

