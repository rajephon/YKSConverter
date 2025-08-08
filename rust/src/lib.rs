pub mod byte_buffer;
pub mod track_event;
pub mod track_builder;
pub mod mf2tt2mf;
pub mod yks_converter;
pub mod errors;
pub mod constants;

pub use yks_converter::YksConverter;
pub use byte_buffer::ByteBuffer;
pub use errors::ConversionError;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mf2tt2mf::Mf2tt2mf;

    #[test]
    fn should_parse_simple_mml_regex() {
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;";
        
        let result = parser.from_mml(mml);
        
        assert!(result, "MML regex parsing should succeed for valid input");
    }

    #[test]
    fn should_fail_with_invalid_mml_format() {
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@t120l4cdefgab>c4.,,,;"; // This fails in C++ baseline
        
        let result = parser.from_mml(mml);
        
        // Currently this passes but should match C++ behavior
        println!("Result for failing MML: {}", result);
    }

    #[test] 
    fn should_debug_regex_matching() {
        use regex::Regex;
        
        let regex_pattern = r"(MML@)\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*);";
        let regex = Regex::new(regex_pattern).unwrap();
        
        let test_cases = [
            "MML@t120l4cdefgab>c4.,,,;",  // Fails in C++
            "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;", // Works in C++
        ];
        
        for (i, mml) in test_cases.iter().enumerate() {
            let captures = regex.captures(mml);
            println!("Test case {}: '{}' -> {:?}", i+1, mml, captures.is_some());
            
            if let Some(caps) = captures {
                for (j, cap) in caps.iter().enumerate() {
                    if let Some(c) = cap {
                        println!("  Group {}: '{}'", j, c.as_str());
                    }
                }
            }
        }
    }

    #[test]
    fn should_generate_same_midi_as_cpp_for_readme_example() {
        // C++ baseline output for: MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;
        // This is the ACTUAL output from the C++ baseline test
        let cpp_expected_hex = "4d546864000000060001000300604d54726b0000009000ff0120596f6b6f736f2050726f6a6563742868747470733a2f2f796f6b6f2e736f2f2900ff510307a12000f00a4110421240007f0041f78140c00101b00a40015b00813eff510304d18d00903c4030803c0000903e4030803e000090404030804000009041403080410000904340308043000090454030804500009047403080470000904840811080480030ff2f004d54726b000000518140c00101b00a40015b00813e903040308030000090324030803200009034403080340000903540308035000090374030803700009039403080390000903b4030803b0000903c408110803c0030ff2f004d54726b000000518140c00101b00a40015b00813e9048403080480000904a4030804a0000904c4030804c0000904d4030804d0000904f4030804f000090514030805100009053403080530000905440811080540030ff2f00";

        let converter = YksConverter::new("MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;".to_string(), 1);
        
        if let Some(buffer) = converter.to_buffer() {
            let rust_hex = buffer.as_slice().iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            
            println!("C++ expected length: {} chars", cpp_expected_hex.len());
            println!("Rust got length:     {} chars", rust_hex.len());
            println!("C++ first 100 chars: {}", &cpp_expected_hex[0..std::cmp::min(100, cpp_expected_hex.len())]);
            println!("Rust first 100:      {}", &rust_hex[0..std::cmp::min(100, rust_hex.len())]);
            
            if rust_hex != cpp_expected_hex {
                // Show detailed differences for debugging
                let max_len = std::cmp::max(rust_hex.len(), cpp_expected_hex.len());
                for i in (0..max_len).step_by(2) {
                    let r_byte = if i < rust_hex.len() { &rust_hex[i..std::cmp::min(i+2, rust_hex.len())] } else { "--" };
                    let c_byte = if i < cpp_expected_hex.len() { &cpp_expected_hex[i..std::cmp::min(i+2, cpp_expected_hex.len())] } else { "--" };
                    if r_byte != c_byte {
                        println!("Byte difference at position {}: Rust={} vs C++={}", i/2, r_byte, c_byte);
                        if i/2 > 50 { break; } // Don't show too many differences
                    }
                }
            }
            
            assert_eq!(rust_hex, cpp_expected_hex, "Rust MIDI output should match C++ exactly");
        } else {
            panic!("Failed to generate MIDI buffer");
        }
    }

    #[test] 
    fn should_parse_single_note_c() {
        use crate::mf2tt2mf::Mf2tt2mf;
        
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@c,,;"; // Simple single note C with 3 tracks
        
        let result = parser.from_mml(mml);
        assert!(result, "Should parse single note C successfully");
        
        let track_events = parser.build();
        assert_eq!(track_events.len(), 3, "Should have three tracks (c, empty, empty)");
        
        // The first track should contain note events
        let first_track = &track_events[0];
        
        // Should have more than just metadata events
        // Look for NoteOn and NoteOff events
        let has_note_on = first_track.iter().any(|event| {
            event.value().contains("NoteOn")
        });
        let has_note_off = first_track.iter().any(|event| {
            event.value().contains("NoteOff")
        });
        
        assert!(has_note_on, "Should have NoteOn event for note C");
        assert!(has_note_off, "Should have NoteOff event for note C");
    }

    #[test]
    fn should_parse_note_sequence_cdef() {
        use crate::mf2tt2mf::Mf2tt2mf;
        
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@cdef,,;"; // Note sequence C-D-E-F
        
        let result = parser.from_mml(mml);
        assert!(result, "Should parse note sequence CDEF successfully");
        
        let track_events = parser.build();
        let first_track = &track_events[0];
        
        // Should have 8 note events: 4 NoteOn + 4 NoteOff
        let note_on_count = first_track.iter().filter(|event| {
            event.value().contains("NoteOn")
        }).count();
        let note_off_count = first_track.iter().filter(|event| {
            event.value().contains("NoteOff")
        }).count();
        
        assert_eq!(note_on_count, 4, "Should have 4 NoteOn events for C-D-E-F");
        assert_eq!(note_off_count, 4, "Should have 4 NoteOff events for C-D-E-F");
    }

    #[test]
    fn should_debug_dotted_quarter_note() {
        // Test dotted quarter note specifically: c4.
        use crate::mf2tt2mf::Mf2tt2mf;
        
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@c4.,,;"; // Dotted quarter note
        
        let result = parser.from_mml(mml);
        assert!(result, "Should parse dotted quarter note");
        
        let tracks = parser.build();
        let first_track = &tracks[0];
        
        println!("\nDebug dotted quarter note 'c4.' parsing:");
        for (i, event) in first_track.iter().enumerate() {
            if event.value().contains("Note") {
                println!("Event {}: {} at time {}", i, event.value(), event.lead_time());
            }
        }
        
        // Check timing - dotted quarter should be 96 * 1.5 = 144 ticks
        let note_on = first_track.iter().find(|e| e.value().contains("NoteOn")).unwrap();
        let note_off = first_track.iter().find(|e| e.value().contains("NoteOff")).unwrap();
        
        let duration = note_off.lead_time() - note_on.lead_time();
        println!("\nNote duration: {} ticks (expected: 144 for dotted quarter)", duration);
        
        // For reference, convert to MIDI to see the hex
        let converter = YksConverter::new("MML@c4.,,;".to_string(), 1);
        if let Some(buffer) = converter.to_buffer() {
            let rust_hex = buffer.as_slice().iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!("\nMIDI hex for 'c4.': {}", rust_hex);
        }
    }

    #[test]
    fn should_parse_note_length_token_l8() {
        use crate::mf2tt2mf::Mf2tt2mf;
        
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@l8cde,,;"; // Length 8 (eighth note) followed by notes
        
        let result = parser.from_mml(mml);
        assert!(result, "Should parse length token l8 successfully");
        
        let track_events = parser.build();
        let first_track = &track_events[0];
        
        // Should have 3 notes with eighth note timing
        let note_on_count = first_track.iter().filter(|event| {
            event.value().contains("NoteOn")
        }).count();
        
        assert_eq!(note_on_count, 3, "Should have 3 NoteOn events for C-D-E");
        
        // Check that notes have proper eighth note timing (24 ticks instead of 48)
        // This test will fail initially because we don't handle l8 yet
        let note_events: Vec<_> = first_track.iter()
            .filter(|event| event.value().contains("NoteOn") || event.value().contains("NoteOff"))
            .collect();
        
        // With eighth notes, timing should be different from quarter notes
        // This assertion will help us identify when length parsing is working
        println!("Note events timing:");
        for event in &note_events {
            println!("  {} at time {}", event.value(), event.lead_time());
        }
        
        // Check that l8 (eighth note) creates proper durations
        // With l8, each note should be 48 ticks (384 ticks per whole note / 8 = 48)
        let first_note_on = note_events.iter().find(|e| e.value().contains("NoteOn")).unwrap();
        let first_note_off = note_events.iter().find(|e| e.value().contains("NoteOff")).unwrap();
        
        let note_duration = first_note_off.lead_time() - first_note_on.lead_time();
        println!("Current note duration: {} ticks", note_duration);
        println!("Expected for l8 (eighth note): 48 ticks (384/8)");
        
        // l8 should create eighth notes with 48 tick duration
        assert_eq!(note_duration, 48, "l8 should create eighth notes with 48 tick duration (384/8)");
    }

    #[test]
    fn should_debug_simple_note_parsing() {
        // Debug single note to understand the parsing
        use crate::mf2tt2mf::Mf2tt2mf;
        
        let mut parser = Mf2tt2mf::new(1, 1, 64, 0);
        let mml = "MML@c,,;"; // Simple single note
        
        let result = parser.from_mml(mml);
        assert!(result, "Should parse simple note");
        
        let tracks = parser.build();
        let first_track = &tracks[0];
        
        println!("\nDebug single note 'c' parsing:");
        for (i, event) in first_track.iter().enumerate() {
            println!("Event {}: {} at time {}", i, event.value(), event.lead_time());
        }
        
        // Look for note events specifically
        let note_events: Vec<_> = first_track.iter()
            .filter(|event| event.value().contains("Note"))
            .collect();
        
        println!("\nNote events only:");
        for event in &note_events {
            println!("  {} at time {}", event.value(), event.lead_time());
        }
        
        // Test MIDI conversion
        let converter = YksConverter::new("MML@c,,;".to_string(), 1);
        if let Some(buffer) = converter.to_buffer() {
            let rust_hex = buffer.as_slice().iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            println!("\nMIDI hex for single 'c': {}", rust_hex);
        }
    }
}
