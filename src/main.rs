#![allow(dead_code)] 

fn read_line() -> i32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.replace("\r", "").replace("\n", "");
    buf.parse::<i32>().unwrap_or(0)
}

fn basic_bit_operation() {
    // let x: u32 = read_line();
    let x: i32 = 88;

    println!("x = {}", x);
    println!("x = {:08b}", x);
    println!("");

    println!("turn off rightmost 1-bit");
    println!("x & (x - 1) = {:08b}", x & (x - 1));
    println!("");

    println!("isolate rightmost 1-bit");
    println!("x & (-x) = {:08b}", x & (-x));
    println!("");

    let x2: i32 = 0b10100111;
    println!("x2 = {:08b}", x2);
    println!("isolate rightmost 0-bit");
    println!("!x2 & (x2 + 1) = {:08b}", !x2 & (x2 + 1));
    println!("");

    println!("mask that identifies the trailing 0’s,");
    println!("producing all 1’s if x = 0 (e.g., 01011000 ⇒ 00000111)");
    println!("!x & (x - 1) | !(x | -x) | (x & -x) - 1 = {:08b}", !x & (x - 1) | !(x | -x) | (x & -x) - 1);
    println!("");

    println!("mask that identifies the rightmost 1-bit and the trailing 0’s,");
    println!("producing all 1’s if x = 0 (e.g., 01011000 ⇒ 00001111)");
    println!("x ^ (x - 1) = {:08b}", x ^ (x - 1));
    println!("");


    println!("right-propagate the rightmost 1-bit");
    println!("x | (x - 1) = {:08b}", x | (x - 1));
    println!("");

    let mut x3 = 90;
    println!("x = {}", x3);
    println!("{:08b}", x3);
    println!("x | (x - 1) = {:08b}", x3 | (x3 - 1));

    x3 = 64;
    println!("x = {}", x3);
    println!("{:08b}", x3);
    println!("x | (x - 1) = {:08b}", x3 | (x3 - 1));
    println!("");

    println!("turn off the rightmost contiguous string of 1-bits");
    println!("((x | (x - 1)) + 1) & x = {:08b}", ((x | (x - 1)) + 1) & x);

    let mut x4 = 70;   
    println!("x = {}", x4);
    println!("{:08b}", x4);
    println!("((x | (x - 1)) + 1) & x = {:08b}", ((x4 | (x4 - 1)) + 1) & x4);

    x4 = 118;
    println!("x = {}", x4);
    println!("{:08b}", x4);
    println!("((x | (x - 1)) + 1) & x = {:08b}", ((x4 | (x4 - 1)) + 1) & x4);

    x4 = 112;
    println!("x = {}", x4);
    println!("{:08b}", x4);
    println!("((x | (x - 1)) + 1) & x = {:08b}", ((x4 | (x4 - 1)) + 1) & x4);
    
    x4 = 114;
    println!("x = {}", x4);
    println!("{:08b}", x4);
    println!("((x | (x - 1)) + 1) & x = {:08b}", ((x4 | (x4 - 1)) + 1) & x4);
    println!("");

    let mut y = x;
    for _  in 0..4 {
        println!("current x = {:08b}", y);
        println!("turn on the rightmost 0-bit");
        println!("x | (x + 1) = {:08b}", y | (y + 1));
        y = y | (y + 1);
    }

}

fn counting_bit() {
    let a = 0x55555555;
    println!("0x55555555 = {}", a);
    println!("{:032b}", a);

}

fn pop(mut x: u32) -> u32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333); 
    x = (x + (x >> 4)) & 0x0F0F0F0F; 
    x = x + (x >> 8); 
    x = x + (x >> 16); 
    x & 0x0000003F
}

fn main() {
    // basic_bit_operation();
    counting_bit();
    println!("254 = {}", pop(254));
    println!("255 = {}", pop(255));
    println!("256 = {}", pop(256));
}
