use std::fs::File;
use std::io::Write;
use yks_converter::{YksConverter, ByteBuffer};

fn write_buffer_to_file(buffer: &ByteBuffer, filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(buffer.as_slice())?;
    println!("Generated: {} (size: {} bytes)", filename, buffer.size());
    Ok(())
}

fn print_buffer_hex(buffer: &ByteBuffer, name: &str) {
    println!("{} hex dump:", name);
    for (i, byte) in buffer.as_slice().iter().enumerate() {
        print!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
    println!();
}

fn main() {
    println!("=== YKSConverter Rust Baseline Tests ===");
    
    // Test Case 1: Simple single track
    println!("Test 1: Simple single track");
    let mml1 = "MML@t120l4cdefgab>c4.,,;".to_string();
    let converter1 = YksConverter::new(mml1, 1);
    if let Some(buffer1) = converter1.to_buffer() {
        write_buffer_to_file(&buffer1, "test1_simple_single_rust.midi").unwrap();
        print_buffer_hex(&buffer1, "Test1_Rust");
    }
    
    // Test Case 2: Multi-track ensemble  
    println!("Test 2: Multi-track ensemble");
    let mml2 = vec![
        "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;".to_string(),
        "MML@t180l8>ccccccc4,l8>eeeeeee4,l8>ggggggg4;".to_string(),
    ];
    let inst2 = vec![26, 74];
    let converter2 = YksConverter::new_multi(mml2, inst2);
    if let Some(buffer2) = converter2.to_buffer() {
        write_buffer_to_file(&buffer2, "test2_multi_track_rust.midi").unwrap();
        print_buffer_hex(&buffer2, "Test2_Rust");
    }
    
    // Test Case 3: Complex MML from README
    println!("Test 3: README example");
    let mml3 = "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;".to_string();
    let converter3 = YksConverter::new(mml3, 1);
    if let Some(buffer3) = converter3.to_buffer() {
        write_buffer_to_file(&buffer3, "test3_readme_example_rust.midi").unwrap();
        print_buffer_hex(&buffer3, "Test3_Rust");
    }
    
    // Test Case 4: Edge case - empty tracks
    println!("Test 4: Edge case with empty tracks");
    let mml4 = "MML@t120l4cde,,;".to_string();
    let converter4 = YksConverter::new(mml4, 1);
    if let Some(buffer4) = converter4.to_buffer() {
        write_buffer_to_file(&buffer4, "test4_empty_tracks_rust.midi").unwrap();
        print_buffer_hex(&buffer4, "Test4_Rust");
    }
    
    println!("=== Rust baseline tests completed ===");
}