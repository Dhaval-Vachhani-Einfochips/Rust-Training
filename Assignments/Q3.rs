fn main() 
{
    let text = "Dhaval";
    let first_char = get_first_character(text);
    assert_eq!(first_char, Some('D'));
}

fn get_first_character(text: &str) -> Option<char>
{
    if text.len() == 0 
    {
        None
    } 
    else
    {
        Some(text.chars().nth(0).unwrap())
    }
}