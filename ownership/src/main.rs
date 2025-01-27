/*
    ----- Ownership Rules -----
    1. Each value in Rust has a variable that's called its owner.
    2. There can onlly be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    println!("Hello World");
    a();

    {
        //  Can be extendable
        let mut s = String::from("Hi");
        println!("{}",s);
        s = String::from("ssdsdsdsd");
        println!("{}",s);
    }

    //  Pointer s1 has MOVE to s2
    let s1 = String::from("USM");
    //  -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1;
    //       -- value moved here

    // println!("{}",s1);
    //            ^^ value borrowed here after move

    let s1 = String::from("USM");
    let s2 = s1.clone(); // Copy
    println!("{}",s1);

    let s = String::from("Pointer");
    takes_ownership(s.clone());
    println!("{}",s);

    fn takes_ownership(s:String){
        //               ^^^^^^ this parameter takes ownership of the value
        println!("{}",s);
    }

    // let s1 = String::from("Moshi Moshi");
    // let (s2, len) =calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // fn calculate_length(s:String) -> (String, usize){
    //     let length = s.len();

    //     (s,length)
    // }

    let s1 = String::from("Moshi Moshi");
    let len =calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s:&String) ->  usize{
        let length = s.len();

        length
    }

    let mut s1 = String::from("hey there");
    change(&mut s1);
    println!("{}",s1);

    fn change(s:&mut String){
        s.push_str(", my girl");
    }

    fn a(){
        let x = "hello";
        let _y = 1;
        println!("{}",x);
        b();
    }

    fn b(){
        let x = String::from("world");
        println!("{}",x);
    }

//     let reference_to_nothing = dangle();

//     fn dangle() -> &String{
//         let s = String::from("sd");
//         &s
// //      ^^ returns a reference to data owned by the current function
//     }
    let  s = String::from("Hello World");
    // let word = first_word(&s);


    // fn first_word(s:&String) -> usize{
    //     let bytes = s.as_bytes();

    //     for (i,&item) in bytes.iter().enumerate(){
    //         if item == b' '{
    //             return i;
    //         }
    //     }
    // }
    let mut s = String::from("Hello World");
    // let hello = &s[..5];
    // let world = &s[6..];

    let word = first_word(&s);
    let s2 = "My name";
    println!("{}", word);

    let word = first_word(&s2);
    println!("{}", word);

    fn first_word(s:&str) -> &str{
        let bytes = s.as_bytes();

        for (i,&item) in bytes.iter().enumerate(){
            if item == b' '{
                return &s[0..i];
            }
        }
        &s[..]
    }
    s.clear();

    let a = [1,2,3,4,5];
    let slice = &a[2..];
    println!("{:#?}",slice);
}