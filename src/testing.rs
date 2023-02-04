
// mod test {
//     #[test]
//     fn it_works(){
//         let result= 2+ 2;
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another(){
//         panic!("Paniced !!");
//     }
    
// }

pub fn print_and_return_value(a: i32) ->  i32{
    println!("hello got value: {}", a);
    a
}

// mod test2 {
//     use super::*;

//     #[test]
//     fn it_should_pass(){
//         let data = 10;
//         let expected = data;

//         assert_eq!(print_and_return_value(data), expected);

//     }

//     #[test]
//     fn it_should_not_pass(){
//         let data = 10;
//         let expected = data;

//         assert_ne!(print_and_return_value(data), expected);

//     }

// }


