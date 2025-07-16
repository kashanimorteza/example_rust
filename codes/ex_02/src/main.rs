fn main() {
    
    println!("\nVariable\n----------------------");

    //---Boolean
    println!("\nBoolean\n-----------");
    let var_bool_1: bool = true;
    let var_bool_2: bool = false;
    println!("var_bool_1 = {}" , var_bool_1);
    println!("var_bool_2 = {}" , var_bool_2);

    //---Char
    println!("\nChar\n-----------");
    let var_char_1: char = 'A';
    println!("var_char_1 = {}" , var_char_1);

    //---Integer
    println!("\nInteger\n-----------");
    let var_int_i8: i8 = 127;
    let var_int_i16: i16 = 32767;
    let var_int_i32: i32 = 2147483647;
    let var_int_i64: i64 = 64;
    let var_int_i128: i128 = 128;
    println!("var_int_i8 = {}" , var_int_i8);
    println!("var_int_i16 = {}" , var_int_i16);
    println!("var_int_i32 = {}" , var_int_i32);
    println!("var_int_i64 = {}" , var_int_i64);
    println!("var_int_i128 = {}" , var_int_i128);

    //---Integer unassigned
    println!("\nInteger unassigned\n-----------");
    let var_int_u8: u8 = 255;
    let var_int_u16: u16 = 65535;
    let var_int_u32: u32 = 4294967295;
    let var_int_u64: u64 = 64;
    let var_int_u128: u128 = 128;
    println!("var_int_u8 = {}" , var_int_u8);
    println!("var_int_u16 = {}" , var_int_u16);
    println!("var_int_u32 = {}" , var_int_u32);
    println!("var_int_u64 = {}" , var_int_u64);
    println!("var_int_u128 = {}" , var_int_u128);

    //---Size
    println!("\nSize\n-----------");
    let var_size_i: isize = 127;
    println!("var_size_i = {}" , var_size_i);
    let var_size_u: usize = 255;
    println!("var_size_u = {}" , var_size_u);

    //---Float
    println!("\nFloat\n-----------");
    let var_float_f32: f32 = 12.222;
    let var_float_f64: f64 = 11.22;
    println!("var_float_f32 = {}" , var_float_f32);
    println!("var_float_f64 = {}" , var_float_f64);

    //---String
    println!("\nString\n-----------");
    let var_string_i: &str = "morteza";
    println!("var_string_i = {}" , var_string_i);

}
