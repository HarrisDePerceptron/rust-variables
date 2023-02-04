/// This will search a query from  the contents
/// 
/// # Example
/// ```
/// use variables::utils::search;
/// let result = search("hello", "hey everyone\nhello world");
/// assert_eq!(result[0], "hello world");
/// 
/// ```
pub fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str>{

    content.lines()
    .filter(|l|l.contains(query))
    .collect()

}