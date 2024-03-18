fn main() 
{
    let data = vec![5,2,8,1];
    let max_value = find_max(data);
    assert_eq!(max_value, Some(8));
}

fn find_max(data:Vec<i32>) -> Option<i32>
{
    if data.len() == 0 
    {
        None
    } 
    else
    {
        let mut max = 0;
        for &i in data.iter() 
        {
            if i > max 
            {
                max = i;
            }
        }
        Some(max)
    }
}