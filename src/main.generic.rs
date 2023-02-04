

mod GEO {
    #[derive(Debug)]
    pub struct Point<T> {
        x: T,
        y: T
    }

    impl<T> Point<T>  {
        pub fn new(x: T, y: T) -> Point<T>{
            Point { x: x, y: y }
    
        }
        pub fn x(&self) -> &T{
            return &self.x;
        }
    
    
    }
}


pub struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T>  {
    pub fn new(x: T, y: T) -> Point<T>{
        Point { x: x, y: y }

    }
    pub fn x(&self) -> &T{
        return &self.x;
    }


}


impl Point<f32> {
    pub fn compute_distaance_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()

    }
}



fn main() {
    println!("hello world ");

    let p = GEO::Point::new(10, 20);


    let pp = Point::new(20,30);


    println!("The point is {:?}", p);
    println!("Value of x ()is {}", p.x());


    let pf = Point::new(30.0f32, 40.0f32); 

    let distance = pf.compute_distaance_origin();
    

    println!("Distance from origin is : {}", distance);
    
    
    
    
}
