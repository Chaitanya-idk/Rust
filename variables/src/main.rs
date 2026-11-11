use std::convert::TryFrom;
use std::io;
fn main() {
    println!("Hello Chaitu !");
    // always take input in terms of string and the convert into required datatype.
    //let mut num = "42".parse().expect("Not a number"); // throws error cause we did not put any datatype infornt of it !.
    let _num: u32 = "42".parse().expect("Not a number");
    //There are 4 main datatypes in rust
    /*
        Number i8,i16,i32,i64,u8,u16,u32,u64
        Floating number f16,f32,f64
        Boolean bool
        Characters char
    */

    let _is_eligible: bool = "true".parse().expect("Not a boolean");
    let mut _name = String::new();
    let _name = "Chaitu".trim().to_string();

    /*
        Type Casting


       fn main() {
           let a: i32 = 100;

           let b: i64 = a as i64;
           let c: u32 = a as u32;
           let d: i16 = a as i16;
           let e: usize = a as usize;

           println!("i64: {}", b);
           println!("u32: {}", c);
           println!("i16: {}", d);
           println!("usize: {}", e);
       }
       fn main() {
            let a: i16 = 300;
            let b: i8 = a as i8;
            println!("{}", b);
        }

    */

    let a: i32 = 300;
    let b: i8 = match i8::try_from(a) {
        Ok(num) => num,
        Err(_) => {
            println!("We cannot convert this !");
            0 //return;
        }
    };
    println!("{}", b);
    /*
        fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
         */

    //compound types -> grouping multiple datatypes into single one is compounding
    //Tuples

    let _new_tuple: (u32, u32, String) = (500, 250, "Chaitanya".to_string());
    println!("{:?}", _new_tuple);
    println!("{:#?}", _new_tuple);

    let mut ages: [u32; 5] = [0; 5];
    for i in 0..5 {
        let mut input = String::new();
        println!("Enter age of {} person.", i + 1);
        io::stdin().read_line(&mut input).expect("No Input");
        ages[i] = input.trim().parse().expect("Invalid age");
    }
    println!("{:?}", ages);
}
