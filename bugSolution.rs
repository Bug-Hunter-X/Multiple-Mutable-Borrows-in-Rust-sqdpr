fn main() {
    let mut x = 5;
    { //Creating a new scope
        let y = &mut x; 
        *y = 10;
    }
    { //Creating a new scope
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}