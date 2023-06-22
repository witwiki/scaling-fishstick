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


}



// Enum













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
