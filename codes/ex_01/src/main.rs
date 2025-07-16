fn main() {
    println!("Hello, world!");

    println!("A");
    println!("A");
    print!("B");
    print!("B");
    print!("\nc\n");


    print!("this my key \"key\"");
    print!("\n");

    print!("this my address \\root\\aaa\\bbbb");
    print!("\n");

    print!(r"this my address \root\aaa\bbbb");
    print!("\n");

    eprintln!("this is for error");

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
