use std::collections::HashMap;

use std::env::current_exe;
use std::io::{ErrorKind, Read};

use std::error::Error;
use std::fs::read_dir;
use std::fs::{File, FileType};

#[derive(Clone)]
struct SearchResultDirectories {
    content: Option<Vec<String>>,
    current_path: String,
    depth: usize,
    directories: Option<Vec<String>>
}

fn main() {
    // panic!("crash and burn this");

    let v = vec![1, 2, 3];

    println!("printing vector: {:?}", v);

    let path = "/home/ai/slack.temp";

    let res = File::open(path);

    let mut f = match res {
        Ok(f) => {
            println!("read the file");
            f
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => println!("not found yo"),
                _ => println!("other error"),
            };

            panic!("error coult not read the file: {}", e.to_string());
        }
    };

    let mut buff = String::new();

    let rr = f
        .read_to_string(&mut buff)
        .expect("could not read from file");

    println!("The len of bytes are {}", rr);

    println!("File \n {}", buff);

    let dir = String::from("/home/ai");


    let max_depth: usize = 3;

    let mut directories = read_directories(&dir).unwrap();

    let files = read_files(&dir, Some("png")).unwrap();


    let mut search_result = SearchResultDirectories {
        content: Some(files),
        current_path: dir.clone(),
        depth: 0,
        directories: Some(directories) 
    };


    let mut search_results: Vec<SearchResultDirectories> = Vec::new();
    search_results.push(search_result.clone());


    let mut search: Vec<SearchResultDirectories> = Vec::new();
    search.push(search_result.clone());

    


    
   loop {

        let mut current_search = match search_results.pop() {
            Some(e) => e,
            None => break

        };
        
        if current_search.depth >= max_depth {
            break;
        }


        let directories = current_search.directories.as_ref().unwrap();

       for i in 0..directories.len(){
           let  mut current_directory = &directories[i];
           let mut directories = read_directories(&current_directory).unwrap();
           let files = read_files(&current_directory, Some("png")).unwrap();
    
           let mut sres = SearchResultDirectories {
               content: Some(files.clone()),
               current_path: current_directory.clone(),
               depth: current_search.depth + 1,
               directories: Some(directories.clone()) 
           };

           search_results.push(sres.clone());
           search.push(sres.clone());            
           
       }
   }

   for i in search { 

        let f = i.content.unwrap();
        
        if f.len() > 0 {

            println!("{:?}", f);
        }

   }


   


}

fn read_directories(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result: Vec<String> = Vec::new();

    let res = read_dir(&path)?;
    for i in res {
        let d = i?;

        let file_name = &d.file_name();
        let file_name_str = match file_name.to_str() {
            Some(s) => s,
            None => continue,
        };

        let file_type = &d.file_type()?;

        let file_path = format!("{path}/{file_name_str}");

        if file_type.is_dir() {
            result.push(file_path);
        }
    }

    return Ok(result);
}

fn read_files(path: &str, ext: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result: Vec<String> = Vec::new();

    let res = read_dir(&path)?;
    for i in res {
        let d = i?;

        let file_name = &d.file_name();
        let file_name_str = match file_name.to_str() {
            Some(s) => s,
            None => continue,
        };

        let file_type = &d.file_type()?;

        let file_path = format!("{path}/{file_name_str}");

        if file_type.is_file() {
            let splits = file_path.split(".");
            let last = splits.last().unwrap_or("");

            if let Some(e) = ext {
                if last == e {
                    result.push(file_path);
                }
            } else {
                result.push(file_path);
            }
        }
    }

    return Ok(result);
}
