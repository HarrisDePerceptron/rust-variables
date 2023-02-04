use std::{fs::File, io::{ErrorKind, Read}, env, error::Error, collections::HashSet};


fn main() {
    println!("hello world ");


    let path = std::env::args().nth(1).expect("Please provide a file path argument");
    
    let what_does_it_say = read_file(&path).unwrap();
    println!("what does it say:\n{}", what_does_it_say);

    let size = what_does_it_say.as_bytes().len();
    let kbs = size as f32  / (1024.0);
    println!("Total Size(KB): {kbs}");


    let splits = split_words(&what_does_it_say);

    for s in splits {
        println!("Split: `{}`", s);
    }


    
}



fn read_file(path: &str) -> Result<String, std::io::Error> {

    let mut f =  match File::open(&path) {
        Ok(f) => f,
        Err (err) => {
        if err.kind() == ErrorKind::NotFound {
            println!("Not found yo! Put in something that exists!!!");
            
        }
        return Err(err);
    }
   };

   let mut buff =  String::new();
    match f.read_to_string(&mut buff) {
        Ok(sz) => sz,
        Err(err) => return Err(err),
    };

   return Ok(buff);
    
}

fn split_words (s: &str)-> Vec<String>{
    let t = String::from(s);
    let t = t.replace('\n', "");


   let col: Vec<&str> = t.split(" ").collect();
    let mut result : Vec<String> = Vec::new();

    for i in col {
        let s = String::from(i);
        if s.len() == 0 {
            continue;
        }
        result.push(s);        
    }
    
    return result;
}


