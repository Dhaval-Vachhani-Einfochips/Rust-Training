fn main()
{
    let var = 100;
    let ref_var = &var; // provides borrow access to data without ownership
    let box_var = Box::new(var); // provides data ownership
    
    println!("Dereferenced value(reference) = {} ", *ref_var);
    println!("Dereferenced value(smart pointer) = {} ", *box_var);
}