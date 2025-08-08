use yks_converter::YksConverter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let mml = &args[1];
        let converter = YksConverter::new(mml.to_string(), 1);
        
        if let Some(buffer) = converter.to_buffer() {
            std::fs::write("output.midi", buffer.as_slice()).unwrap();
            println!("Generated output.midi ({} bytes)", buffer.size());
        } else {
            eprintln!("Failed to convert MML to MIDI");
            std::process::exit(1);
        }
    } else {
        println!("YKS Converter - MML to MIDI converter");
        println!("Usage: cargo run -- \"MML@t120l4cdefgab>c4.,,;\"");
        println!();
        println!("Examples:");
        println!("  cargo run -- \"MML@t120l4cdefg,,;\"");
        println!("  cargo run -- \"MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;\"");
    }
}
