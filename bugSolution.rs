fn main() {
    let mut v = vec![1, 2, 3];
    let mut value = 10; 
    { // Scope is important here
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = value; // Assign value within the scope 
    }
    println!("v: {:?}", v);
}
    value = 100;
}