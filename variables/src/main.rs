fn main() {
    let  x = 5;
    println!("The value of x is {}", x);
    let x = "six";
    println!("The value of x is {}", x);

    const PI: f64 = 3.1415926535;
    println!("The value of PI is {}", PI);

    let n:i8 = -128;

    let tup = ("Lanny", 12,43);
    let (name, _n1,n2) = tup;
    let n1 = tup.1;

    let error_codes = [200, 404,500];
    let not_found = error_codes[1];
    println!("{} {} {}", name,n1,n2);

    if not_found == 404{
        println!("NOT FOUND");
    }

    let byte = [0;8];
    for b in &byte {
        print!("{} ", b);
    }
    println!();

    let mut counter:u8 =  3;
    while counter > 0{
        println!("{}!", counter);
        counter-=1;

    }
    println!("LIFT OFF!");

    print_hello();
    println!("{}",sum(32,32));
    swap(1,2);

    let elements = [1,2,3,4];

    for element in  elements.iter(){
        print!("{element} ");
    }
    println!();

}

fn print_hello(){
    println!("Hello");
}

fn sum(a:i32,b:i32) ->i32{
     return a+b;
}

fn swap(mut a: u32, mut b: u32) {
    println!("a:{} b:{}",a,b);
    a = a ^ b;
    b = a ^ b;
    a = a ^ b;
    println!("a:{} b:{}",a,b);
}