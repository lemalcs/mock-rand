// Generates a f32 value between 0 and 1 from a u8 number (maximum value: 255).
fn mock_rand(n: u8) -> f32 {
    (n as f32) / 255.0
}

// Generates a f32 value between 0 and 1 without doing a division which is slow.
fn mock_rand_2(n: u8) -> f32 {
    // Exponent of -1 represented as 126 (in base 10) which is one of the three fields
    // of and floating-point number in Rust
    let base: u32 = 0b0_01111110_00000000000000000000000; // binary format for code
    println!("base = {:08b}", base); // 111111000000000000000000000000

    // Move `n` 15 places to the left (empty spaces are filled with zeros)
    let large_n = (n as u32) << 15; 
    println!("before shifting bits of large_n = {:08b}", (n as u32)); // 00111010
    println!("after shifting bits of large_n = {:08b}", large_n);     // 111010000000000000000
    
    // The `other_base` variable has the same value as `base` variable
    // although written with more underscores and the latter
    let other_base: u32 = 0b0011_1111_0000_0000_0000_0000_0000_0000;
    println!("other_base = {:08b}", other_base);
    
    // Takes a bitwise OR
    let f32_bits = base | large_n;
    println!("f32_bits = {:08b}", f32_bits); // 111111000111010000000000000000

    // OR operation
    //111111000000000000000000000000
    //         111010000000000000000

    let m = f32::from_bits(f32_bits); // Interprets u32 as f32

    2.0 * (m - 0.5)
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
    
    println!("other input range: {:08b} -> {:?}", 0x3a, mock_rand(0x3a));
    println!("other input range: {:08b} -> {:?}", 0x3a, mock_rand_2(0x3a));
}
