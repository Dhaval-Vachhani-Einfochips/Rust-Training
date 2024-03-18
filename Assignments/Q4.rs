fn main() 
{
    let day = 3;
    let day_type = get_day_type(day);
    println!("{}", day_type);
    assert_eq!(day_type, "Weekday");
}

fn get_day_type(day:u8) ->&'static str
{
    let day_type = match day {
        1 | 2 | 3 | 4 | 5 | 6 => "Weekday",
        7 => "Weekend",
        _ => "Invalid day"
    };
    
    return day_type;
}