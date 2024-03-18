fn main() 
{
    let name = "Dhaval";
    let greeting = greet(name);
    println!("{}", greeting);
    assert_eq!(greeting, "Hello, Dhaval!");
}

fn greet(name:&str) -> String
{
    let ret_str = format!("Hello, {}!", name);
    return ret_str;
}
