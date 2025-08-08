use crate::track_event::*;
use crate::track_builder::TrackBuilder;
use regex::Regex;
use std::collections::HashMap;

const MIN_VOLUME: u8 = 1;
const MAX_VOLUME: u8 = 15;
const MAX_OCTAVE: u8 = 9;

pub struct Mf2tt2mf {
    channel: u8,
    instrument: u8,
    pan: u8,
    reverb: u8,
    min_note: u8,
    max_note: u8,
    sound_map: HashMap<char, i32>,
    track_builders: Vec<TrackBuilder>,
}

impl Mf2tt2mf {
    pub fn new(channel: u8, instrument: u8, pan: u8, reverb: u8) -> Self {
        let mut sound_map = HashMap::new();
        sound_map.insert('c', 0);
        sound_map.insert('d', 2);
        sound_map.insert('e', 4);
        sound_map.insert('f', 5);
        sound_map.insert('g', 7);
        sound_map.insert('a', 9);
        sound_map.insert('b', 11);
        sound_map.insert('C', 0);
        sound_map.insert('D', 2);
        sound_map.insert('E', 4);
        sound_map.insert('F', 5);
        sound_map.insert('G', 7);
        sound_map.insert('A', 9);
        sound_map.insert('B', 11);

        Mf2tt2mf {
            channel,
            instrument,
            pan,
            reverb,
            min_note: 0,
            max_note: 96,
            sound_map,
            track_builders: Vec::new(),
        }
    }

    pub fn from_mml(&mut self, mml: &str) -> bool {
        self.track_builders.clear();
        
        let regex_pattern = r"(MML@)\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*),\s*([\s0-9a-glnortvA-GLNORTV#<>.&+-]*);";
        let regex = match Regex::new(regex_pattern) {
            Ok(r) => r,
            Err(_) => {
                eprintln!("Regex compile failed");
                return false;
            }
        };

        let captures = match regex.captures(mml) {
            Some(caps) => caps,
            None => {
                eprintln!("Regex parse failed");
                return false;
            }
        };

        let mut track_list = Vec::new();
        
        for i in 1..captures.len() {
            if let Some(track_match) = captures.get(i) {
                let track = track_match.as_str();
                if !track.starts_with("MML@") {
                    track_list.push(track.to_string());
                }
            }
        }

        if track_list.is_empty() {
            eprintln!("Track is empty");
            return false;
        }

        let ch = self.channel;
        let inst = self.instrument;
        let pan = self.pan;
        let reverb = self.reverb;

        for (i, track) in track_list.iter().enumerate() {
            let mut builder = TrackBuilder::new(ch);
            
            if ch == 1 && i == 0 {
                let sys_ex_data = vec![0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7f, 0x00, 0x41, 0xf7];
                
                let meta_text = Box::new(MetaText::new("Yokoso Project(https://yoko.so/)".to_string()));
                let tempo = Box::new(Tempo::new(500000));
                let sys_ex = Box::new(SysEx::new(sys_ex_data));
                
                builder.put_event(meta_text);
                builder.put_event(tempo);
                builder.put_event(sys_ex);
            }

            let mut prog_change = Box::new(ProgramChange::new(ch, inst));
            prog_change.set_lead_time(192);
            builder.put_event(prog_change);

            let mut pan_control = Box::new(ControlChange::new(ch, 10, pan));
            pan_control.set_lead_time(193);
            builder.put_event(pan_control);

            let mut reverb_control = Box::new(ControlChange::new(ch, 91, reverb));
            reverb_control.set_lead_time(194);
            builder.put_event(reverb_control);

            if !track.is_empty() {
                let track_events = self.parse_track(track, 384);
                builder.put_events(track_events);
            } else {
                let mut end_track = Box::new(EndOfTrack::new());
                end_track.set_lead_time(385);
                builder.put_event(end_track);
            }

            self.track_builders.push(builder);
        }

        true
    }

    fn parse_track(&self, track: &str, lead_time: u32) -> Vec<Box<dyn TrackEvent>> {
        use regex::Regex;
        
        let mut events: Vec<Box<dyn TrackEvent>> = Vec::new();
        let mut delta_time = lead_time;
        
        // C++ algorithm state variables
        let mut note_time = 96u32; // Current note duration (quarter note = 96 ticks)
        let mut octave = 4i32;     // Current octave
        let mut volume = 8i32;     // Current volume (1-15)
        let mut curr_note = 0i32; // For tie processing
        let mut is_tied = false; // Tie state
        
        // C++ time constants
        let semibreve = note_time * 4; // Whole note = 384 ticks
        let minim = note_time * 2;     // Half note = 192 ticks
        
        // Step 1: Remove whitespace (C++ line 115-116)
        let clean_track = track.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        
        // Step 2: Extract all MML tokens using exact C++ regex pattern (line 118)
        let token_regex = Regex::new(r"[OTLVNRA-Gotlvnra-g<>][\+\-\#]?[0-9]*\.?&?").unwrap();
        let tokens: Vec<&str> = token_regex.find_iter(&clean_track).map(|m| m.as_str()).collect();
        
        // Step 3: Process tokens in order (C++ lines 130-259)
        for token in tokens {
            // Parse control tokens first (length, octave, tempo, volume, octave shift)
            let control_regex = Regex::new(r"([lotvLOTV<>])([1-9][0-9]*|0?)(\.?)(&?)").unwrap();
            if let Some(caps) = control_regex.captures(token) {
                let op = caps.get(1).unwrap().as_str().to_lowercase();
                let value_str = caps.get(2).map_or("", |m| m.as_str());
                let dot = caps.get(3).map_or("", |m| m.as_str());
                let _ampersand = caps.get(4).map_or("", |m| m.as_str()); // For ties
                
                let value = if value_str.is_empty() { 0i32 } else { value_str.parse::<i32>().unwrap_or(0) };
                
                match op.as_str() {
                    "l" => {
                        // Length token (C++ lines 142-147)
                        if value > 0 && value <= minim as i32 {
                            note_time = semibreve / value as u32; // C++ formula: floor(semibreve/value)
                            if dot == "." {
                                note_time = (note_time as f32 * 1.5) as u32; // Dotted notes
                            }
                            // Handle ties (&) - for now just mark the flag
                            if _ampersand == "&" {
                                is_tied = true;
                            }
                        }
                    }
                    "o" => {
                        // Octave token (C++ lines 148-149)
                        octave = value;
                    }
                    "t" => {
                        // Tempo token (C++ lines 164-167)
                        if value > 0 {
                            let tempo_microseconds = (60_000_000 / value) as u32; // Use integer division like C++
                            let mut tempo_event: Box<dyn TrackEvent> = Box::new(Tempo::new(tempo_microseconds));
                            tempo_event.set_lead_time(delta_time);
                            events.push(tempo_event);
                        }
                    }
                    "v" => {
                        // Volume token (C++ lines 168-174)
                        if value < 1 {
                            volume = 1;
                        } else if value > 15 {
                            volume = 15;
                        } else {
                            volume = value;
                        }
                    }
                    "<" => {
                        // Octave down (C++ lines 175-180)
                        if octave <= 0 {
                            octave = 0;
                        } else {
                            octave -= 1;
                        }
                    }
                    ">" => {
                        // Octave up (C++ lines 181-186)  
                        if octave >= 9 {
                            octave = 9;
                        } else {
                            octave += 1;
                        }
                    }
                    _ => {}
                }
            } else {
                // Parse note/rest tokens (C++ lines 188-257)
                let note_regex = Regex::new(r"([a-gnA-GN])([\+\#-]?)([0-9]*)(\.?)(&?)").unwrap();
                if let Some(caps) = note_regex.captures(token) {
                    let note_char = caps.get(1).unwrap().as_str().to_lowercase();
                    let pitch = caps.get(2).map_or("", |m| m.as_str());
                    let length_str = caps.get(3).map_or("", |m| m.as_str());
                    let dot = caps.get(4).map_or("", |m| m.as_str());
                    let ampersand = caps.get(5).map_or("", |m| m.as_str());
                    
                    let mut note = 0i32;
                    let mut tick = note_time;
                    
                    // Handle 'n' notes differently (C++ lines 196-200)
                    if note_char == "n" {
                        if !length_str.is_empty() {
                            if let Ok(value) = length_str.parse::<i32>() {
                                if value >= 0 && value <= self.max_note as i32 {
                                    note = value;
                                }
                            }
                        }
                    } else {
                        // Regular note processing (C++ lines 201-216)
                        if !length_str.is_empty() {
                            if let Ok(length_val) = length_str.parse::<i32>() {
                                if length_val >= 1 && length_val <= minim as i32 {
                                    tick = semibreve / length_val as u32;
                                }
                            }
                        }
                        if dot == "." {
                            tick = (tick as f32 * 1.5) as u32;
                        }
                        
                        // Calculate MIDI note number from sound map (C++ lines 208-210)
                        if let Some(&base_note) = self.sound_map.get(&note_char.chars().next().unwrap()) {
                            note = (12 * octave) + base_note;
                        }
                        
                        // Apply accidentals (C++ lines 211-215)
                        if pitch == "+" || pitch == "#" {
                            note += 1;
                        } else if pitch == "-" {
                            note -= 1;
                        }
                    }
                    
                    // Clamp to valid range (C++ lines 218-224)
                    while note < self.min_note as i32 { note += 12; }
                    while note > self.max_note as i32 { note -= 12; }
                    note += 12; // Final offset (C++ line 224)
                    
                    // Handle ties and note events (C++ lines 226-241)
                    if is_tied && note != curr_note {
                        is_tied = false;
                        // Generate Note Off for previous tied note
                        let mut note_off: Box<dyn TrackEvent> = Box::new(NoteOff::new(self.channel, curr_note as u8, 0));
                        note_off.set_lead_time(delta_time);
                        events.push(note_off);
                    }
                    
                    if !is_tied {
                        // Generate Note On (C++ line 232)
                        let note_number = note as u8;
                        let velocity = (8 * volume) as u8;
                        let mut note_on: Box<dyn TrackEvent> = Box::new(NoteOn::new(self.channel, note_number, velocity));
                        note_on.set_lead_time(delta_time);
                        events.push(note_on);
                    }
                    
                    delta_time += tick; // Advance time (C++ line 234)
                    
                    if ampersand == "&" {
                        is_tied = true;
                        curr_note = note;
                    } else {
                        is_tied = false;
                        // Generate Note Off (C++ line 240)
                        let mut note_off: Box<dyn TrackEvent> = Box::new(NoteOff::new(self.channel, note as u8, 0));
                        note_off.set_lead_time(delta_time);
                        events.push(note_off);
                    }
                } else {
                    // Handle rest tokens 'r' (C++ lines 246-257)
                    let rest_regex = Regex::new(r"[rR]([0-9]*)(\.?)").unwrap();
                    if let Some(caps) = rest_regex.captures(token) {
                        let mut tick = note_time;
                        let length_str = caps.get(1).map_or("", |m| m.as_str());
                        let dot = caps.get(2).map_or("", |m| m.as_str());
                        
                        if let Ok(length_val) = length_str.parse::<i32>() {
                            if length_val >= 1 && length_val <= minim as i32 {
                                tick = semibreve / length_val as u32;
                            }
                        }
                        if dot == "." {
                            tick = (tick as f32 * 1.5) as u32;
                        }
                        delta_time += tick;
                    }
                }
            }
        }
        
        // Handle remaining tied notes (C++ lines 242-245)
        if is_tied {
            let mut note_off: Box<dyn TrackEvent> = Box::new(NoteOff::new(self.channel, curr_note as u8, 0));
            note_off.set_lead_time(delta_time);
            events.push(note_off);
        }
        
        // Add final note time like C++ (line 260)
        delta_time += note_time;
        
        let mut end_track: Box<dyn TrackEvent> = Box::new(EndOfTrack::new());
        end_track.set_lead_time(delta_time);
        events.push(end_track);
        
        events
    }

    pub fn build_to_string(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("MFile 1 {} 96", self.channel));
        
        for builder in &self.track_builders {
            let build_result = builder.build();
            result.extend(build_result);
        }
        
        result
    }

    pub fn build(&self) -> Vec<Vec<Box<dyn TrackEvent>>> {
        let mut result: Vec<Vec<Box<dyn TrackEvent>>> = Vec::new();
        
        for builder in &self.track_builders {
            let mut track_events: Vec<Box<dyn TrackEvent>> = Vec::new();
            for event in builder.event_list() {
                // Now we can properly clone events using the clone_event method
                track_events.push(event.clone_event());
            }
            result.push(track_events);
        }
        
        result
    }
}