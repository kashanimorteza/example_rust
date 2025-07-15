fn main() {
    
    println!("\nVariable\n----------------------");

    let name: &str = "morteza";
    let family: &str = "kashani";
    let age: i32=40;

    print!("my name is {name}");
    print!("\n");
    print!("my name is {}", name);
    print!("\n");
    print!("my name is {} and {}", name, family);
    print!("\n");
    print!("my name is {} and {} and {}", name, family, age);
    print!("\n");

    let sentence: String = format!("my name is {} and {} and {}", name, family, age);
    println!("{}", sentence);

    let dec_num: i32 = 123;
    println!("my number is {} and in hex is {:#X}", dec_num, dec_num);
    println!("my number is {} and in hex is {:#x}", dec_num, dec_num);

    println!("my number is {} and in hex is {:X}", dec_num, dec_num);
    println!("my number is {} and in hex is {:x}", dec_num, dec_num);

    println!("my number is {} and in hex is {:#b}", dec_num, dec_num);
    println!("my number is {} and in hex is {:b}", dec_num, dec_num);

    println!("my number is {} and in hex is {:#o}", dec_num, dec_num);
    println!("my number is {} and in hex is {:o}", dec_num, dec_num);

}
