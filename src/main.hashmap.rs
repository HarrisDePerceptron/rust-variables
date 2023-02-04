use std::collections::HashMap;

fn main() {

    let mut hm: HashMap<String, u8> = HashMap::new();
    hm.insert(String::from("harris"), 10);
    hm.insert(String::from("hamza"), 20);
    hm.insert(String::from("haider"), 30);


    for i in &hm {
        let (k, v) = i;
        
        println!("k: {k}, v: {v}");
    }
    

    let hv = hm.get("harris").copied().unwrap();
    println!("value for harris: {hv}");
    
    let name = String::from("harris");
    
    let score = hm.get(&name).unwrap_or(&0);

    println!("Score is {score}");

    let key = String::from("sam");
    let value: u8 = 100;

    hm.insert( key, value);
    

    hm.insert(String::from("sam"), 200);


    hm.entry(String::from("sam")).or_insert(255);

    println!("has map is : {:?}", hm);


    let text = String::from("hello world my name is harris. let have a a chit chat over this new program");
    let mut count: HashMap<String, u32> = HashMap::new();


    for c in text.split_whitespace() {
        let res = count.entry(c.to_string()).or_insert(0);
        *res+=1;

    }

    println!("Count hashmap is {:?}", count);

    println!("hi, original text is {}", text);
    




    
}

