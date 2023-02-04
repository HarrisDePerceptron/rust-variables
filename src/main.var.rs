

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f32),
    Text(String)
    
}

fn main() {
    println!("hello wolrd");

    let mut v : Vec<i32>= Vec::new();
    v.push(1);
    v.push(2);


    v.push(100);

    for i in &v {
        println!("Vector elem: {i}");
    }

    println!("vector v: {:?}", v);

    let mut v2 = vec![10, 20 , 30 , 50, 60];
    
    for i in &mut v2 {
        *i *= 20;


    }

    println!("vector v: {:?}", v2);


    let spreadsheet = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(3.2),
        SpreadSheetCell::Text(String::from("hello"))
    ];


    println!("spreadsheet: {:?}",  spreadsheet);


    let chars = Vec::from("123");




}