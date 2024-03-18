fn main() 
{
    let text = "hello";
    let reversed_text = reverse_string(text);
    println!("{}", reversed_text);
    assert_eq!(reversed_text, "olleh");
}

fn reverse_string(text: &str) -> String
{
    let mut rev_str = String::new();
    for c in text.chars().rev()
    {
        rev_str.push(c);
    }
    
    return rev_str;
}