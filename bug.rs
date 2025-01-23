fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &x; 
    println!("{}", *z); //This will print 10
    {        
        let z = &mut x; //This will cause error
        *z = 20;    
    }    
}