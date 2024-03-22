fn main()
{
    let a:i32 = 5;
    let b = Box::new(a);
    println!("value contains by box = {} ", *b);
}