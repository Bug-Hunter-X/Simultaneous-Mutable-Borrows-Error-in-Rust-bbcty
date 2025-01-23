fn main() {
    let mut x = 5;
    {
        let z = &mut x; 
        *z = 20;    
    }
    println!("{}", x); //This will print 20
} 