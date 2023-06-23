use std::collections::HashMap;

fn main() {
    
    // Var Types
    let unsigned: u8 = 102 + 12;
    let signed: i8 = 127 * -1;
    let float: f32 = 1.3 / 2.0;
    println!("unsigned: {}, signed: {}, float: {}", unsigned, signed, float);

    let letter = "v";
    let emogi = "\u{1f600}";
    let is_true = true && false || false;
    println!("letter: {}, emogi: {}, is_true: {}", letter, emogi, is_true);

    // Arrays
    let arr: [u8; 3] = [1, 2, 3];
    let arr2: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[1], arr2.len());
    // print structure of array and other objects
    println!("{:?}", arr2);

    // Tuples are python style
    
    // Functions
    println!("{}", is_even(13));

    // Mutability
    let mut edf = 123;
    println!("{}", edf);
    edf = 232;
    println!("{}", edf);

    // Slices and Arrays
    let arr3 = [0,1,2,3];   //length
    let slice3 = &arr3[1..3];    // [1,2] don't know the length however
    borrowing_slice(arr3, slice3);

    // Strings
    let _rndm_str: &str = "hello verse";
    let mut rndm_str2: String = String::from("Hello verse");

    let slice = &rndm_str2[.. 6];
    let _ = slice.len();

    rndm_str2.push('1');
    rndm_str2.push_str("! multi?");
    rndm_str2 = rndm_str2.replace("Hello", "What");
    println!("{}", rndm_str2);

    // If-Else statements are python style
    // For loops statements are python style
    // While loops statements are python style  (break has no semi-colon like returns)

    // Switch-Case
    let mat = 6;
    match mat {
        0 => println!("0"),
        1 | 2 => println!("1,2"),   // 1 or 2
        3..=4 => println!("3,4"),   // inclusive range
        _ => println!("Default")        
    }

    // Structs
    let nme = String::from("Bird");
    let brd = Bird{name: nme, attack: 34};
    brd.print_name();
    brd.print_attack_score();

    // Traits (these are interfaces. There's no support for OOP, instead interfaces are used which are called Traits)
    println!("{} {}", brd.can_fly(), brd.is_animal());


    // Enum
    let u: MyEnum = MyEnum::A;
    let v: MyEnum = MyEnum::B(5);
    let w: MyEnum = MyEnum::C { x: 10, y: 20 };
    println!("{:?}", u);
    println!("{:?}", v);
    println!("{:?}", w);

    if let MyEnum::B(val) = v {
        println!("{}", val);
    }

    if let MyEnum::C { x, y } = w {
        println!("{} {}", x, y);
    }


    // Vector (Dynamic Arrays default in stdlib of rust)\
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("+++ vec below +++");
    println!("{:?}", vec);
    vec.len();
    println!("{:?}", vec.len());
    vec[0];
    println!("{:?}", vec[0]);
    vec.push(7);
    println!("{:?}", vec);
    vec.remove(0);
    println!("{:?}", vec);

    // Hash Map (Common data structure)
    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi again");
    println!("{:?}", map);

    match map.get(&0) {         // have to reference the index/address of the value in a Hashmap in basic methods
        Some(str1) => println!("{}", str1),
        None => println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str2) => println!("{}", str2),
        _ => println!("Doesn't exist in map"),       
    }

    map.remove(&0);     // have to reference the index/address of the value in a Hashmap in basic methods
    println!("{:?}", map);


    // // Options in a nutshell
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwrapping a 'Some' variant will extract the value wrapped
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a 'None' variant will 'panic!'
    println!("{:?} unwraps to {}", divide2, divide2.unwrap());


    // // Results in a nutshell
    let div = div(4,2);
    // let res = div.expect("we crashed");

    // method 1
    match div {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v)        
    }
    // // method 2
    // if div.is_ok() {
    //     println!("{}", div.unwrap());
    // }
    // // method 3a
    // println!("{}", div.unwrap());
    // // method 3b
    // println!("{}", div.unwrap_or(100));     // kinda like a try/catch/expect where it returns 100 in this case if not unwrapped
    // // method 4
    // println!("{}", res);


}


// // Results in a nutshell
// Err, an enum containing error code
// Ok(value), a wrapper that contains a value
fn div(d1: i32, d2: i32) -> Result<i32, MyError> {
    if d1 % d2 != 0 {
        Err(MyError::Error1)
    } else {
        Ok(d1 / d2)
    }
}

#[derive(Debug)]
enum MyError {
    Error1
}




// // Options in a nutshell
// 'None', to indicate failure or lack of value, and
// 'Some(value)', a tuple struct that wraps a value with type T.
fn divide(divid: i32, divis: i32) -> Option<i32> {
    if divid % divis != 0 {
        None
    } else {
        Some(divid / divis)
    }
}





// Enum
#[derive(Debug)]        // telling compiler how we are printing our data structure
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}    // a struct
}


// Traits (these are interfaces. There's no support for OOP, instead interfaces are used which are called Traits)
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        false
    }
}




// Structs
struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self){
        println!("{}", self.name);
    }

    fn print_attack_score(&self){
        println!("{}", self.attack);
    }
}


// Slices and Arrays
fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);

    
}

// Functions
// all rust functions a private by default unless 'pub' specified.
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0    // return bool (to return a value from a fn you leave the last line without semi colon)
}
