
use variables::utils::search;


struct Config{
    query: String, 
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn'tget any query string")
        };


        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get file path string")
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        
        Ok(Config{
            query: query,
            file_path: file_path,
            ignore_case: ignore_case
        })
        
    }
}


fn main() {
    println!("hello world ");

    let config = Config::build(std::env::args()).unwrap_or_else(|err|{
        panic!("unable to parse arguments: {err}")
    });

    let query = "harris";
    let content = "hello world\nits harris\n how are you mattes";
    let results: Vec<&str> = search(query, content);
    let r = results[0];

    assert_eq!(r, "its harris");
    for i in results.iter(){
        println!("got: {}", i);

    }
}





