//---------------------Main
fn main() {
    //---fn_1
    fn_1();
    //---fn_2
    fn_2(8);
    //---fn_3
    let var_fn_3: i8 = fn_3();
    println!("fn_3 {}", var_fn_3);
    //---fn_4
    let var_fn_4: i8 = fn_4();
    println!("fn_4 {}", var_fn_4);
    //---fn_5
    let var_fn_5: i8 = fn_5(10);
    println!("fn_5 {}", var_fn_5);
}

//---------------------Simple
fn fn_1() {
    println!("fn_1");
}

//---------------------Input
fn fn_2(x: i8) {
    println!("fn_2 {}", x);
}

//---------------------Output
fn fn_3() -> i8 {
    return 16;
}

fn fn_4() -> i8 {
    9
}

//---------------------Input | Output
fn fn_5(x: i8) -> i8 {
    return x;
}
