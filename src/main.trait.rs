

// use aggregator::{Summary, Article, Tweet};


use std::fmt::Display;

use aggregator::{Summary, Article, Tweet};

pub mod aggregator;

impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from(self.body.clone() + "/" + &self.created_at)
    }

    fn summarize_author(&self) -> String {
        String::from("default author")
    }
}


impl Summary for Article {
    fn summarize(&self) -> String {
        String::from(self.name.clone() + &self.body + "/" + &self.description + "/" + &self.created_at)
    }

    fn summarize_author(&self) -> String {
        String::from("default author")
    }
}


impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("i32 value is {}",self.to_string()) 
        
    }

    fn summarize_author(&self) -> String {
        String::from("hello i32")
    }
}


impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        f.write_str(&self.summarize())
    }
}

impl Clone for Tweet {
    fn clone(&self) -> Self {
        Self { body: self.body.clone(), created_at: self.created_at.clone() }
    }
}


struct Pair <T> {
    x: T, 
    y: T
}

impl<T> Pair<T> {
    fn  new(x:T, y: T) -> Self{
        Self {x: x,y: y}
    }
}

impl <T>  Pair <T> 
where 
    T: Display + PartialOrd
{
    fn cmp_display(&self) -> &Self{
        if self.x >  self.y {
            println!("X is greater than y");
        }else {
            println!("y is greater than x");
        }

        return self;
    }   
}





fn main() {
    println!("hello world ");


    let tweet = Tweet {
        body: "this is a tweet about something".to_string(),
        created_at: "10/12/11".to_string()
    };

    let article = Article {
        body: "the article is about rust".to_string(),
        created_at: "10/12/11".to_string(),
        description: "this is a article description".to_string(),
        name: "The Article".to_string()
    };


    println!("Tweet summarty:\n{}", tweet.summarize());

    println!("Article summary:\n{}", article.summarize());


    let arr: Vec<Box<dyn Summary>>  = vec![Box::new(tweet.clone()), Box::new(article)];

//    print_summary(&arr);


//    println!("vec: {:?}" , arr.get(0).unwrap().summarize());


//    let i = 12;
//    println!("{}", i.summarize());

//    notify(&i);

   notifications(&arr);


   println!("The tweet with display is {}", tweet);

   notify_disp(&tweet);

   let p = Pair::new(3, 2);

   p.cmp_display();
    






}



fn print_summary(arr: &Vec<&dyn Summary>) -> (){

    for i in arr {
        println!("Summary is:\n{}", i.summarize());
        i.print();

    }

}


fn notify<T: Summary>(s: &T)->(){
    println!("Notification: {}", s.summarize());

}

fn notifications<T: Summary  + ?Sized> (s: &[Box<T>])->(){
    for i in s {
        println!("Summary is:\n {}", i.summarize());
    }

}

fn notify_disp<T: Summary + Display> (o: &T)-> (){
    println!("Tweet obk: {}", o);
    println!("Tweet summary: {}", o.summarize());

}