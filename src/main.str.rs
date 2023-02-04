

fn main() {
    println!("hello wolrd");
    
    let sl = "hello world";

    output(&sl);

    let s = sl.to_string();

    let  b = s.as_bytes();
    
    for i in b {
        println!("byte: {i}");

    }

    let hello = String::from("السلام عليكم");

    
    println!("hello in urdu: {}", hello);

    let mut s1 = String::from("hello ");
    

    let s2 = String::from("harriss");
    

    s1.push_str(&s2);



    let s3 = s1 + &s2;

    println!("S3: {s3}\n");

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = t1 + "-" + &t2 + "-" + &t3;
    
    println!("TTT: {t}");

    let tf = format!("/{t}--{t2}--{t3}/");

    println!("fotmat macro: {tf}");


    let ss = String::from("hello");
    let l = ss.len();
    println!("String length is {l}");

    let yo = String::from("Здравствуйте");
    println!("String '{}'length is {}",yo,  yo.len());

    for i in yo.chars() {
        println!("chars: {}", i);
        
    }

    let sl  = &yo[0..4];

    println!("slice: {sl}");


    
    
}


fn output(s: &str) {
    println!("output: {s}");

}