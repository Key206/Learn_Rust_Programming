fn main() {
    /*
     * Variables & Mutability
     * In Rust, when we declare a variable without keyword 'mut'
     * That variable will be immutable variable by default (can not change)
     */

    /* Immutable variables */
    /*
     * we also have the basic data types such as: 
     * Integer, Float, Boolean, Char, String, ...
     * 
     * Integer data type: signed & unsigned
     * i8, i16, i32, i64, i128 => signed 
     * u8, u16, u32, u64, u128 => unsigned 
     * 
     * We also have the Signed and unsigned integers, 
     * the same size as an address on the machine (32 or 64 bits) => isize, usize
     * 
     */
    let _x_i32: i32 = -10; 
    let _x_u32: u32 = 10;
    let my_float: f32 = 0.5;
    let _hello: &'static str = "hello";
    let _j: char = 'j';
    let _boolean: bool = true;
    let _my_temp: f32 = my_float;
    /* _x & _e will have data type based on numeric type */
    let _x = 30i8; // _x will have i8 type
    let _e = 30_i32; // _e will have i32 type
    /* size_of_val() will return the size of argument by byte */
    println!("size of val _x = {}", size_of_val(&_x));
    println!("size of val _e = {}", size_of_val(&_e));
    
    /* Mutable variables */
    /*
     * 
     */
    let mut mut_x_u32: u32 = 10;
    println!("Mutable variable mut_x_u32 before change: {mut_x_u32}");
    mut_x_u32 = 20;
    println!("Mutable variable mut_x_u32 after change: {mut_x_u32}");

    if (mut_x_u32 > 20) && (mut_x_u32 != 30) {

    }
    else if mut_x_u32 > 30 {
        
    }
    else {
        
    }

    match mut_x_u32 {
        20 => {
            mut_x_u32 += 1;
            println!("case 20 : {mut_x_u32}");
        }
        10 => {
            mut_x_u32 += 1;
            println!("case 10 : {mut_x_u32}");
        }
        _ => {
            mut_x_u32 += 1;
            println!("case default : {mut_x_u32}");
        }
    }
}
