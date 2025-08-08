//! Constants used throughout the YKS Converter

/// MIDI timing constants
pub mod timing {
    /// Ticks per whole note (semibreve)
    pub const TICKS_PER_WHOLE_NOTE: u32 = 384;
    
    /// Ticks per half note (minim)
    pub const TICKS_PER_HALF_NOTE: u32 = 192;
    
    /// Ticks per quarter note (crotchet) - default note duration
    pub const TICKS_PER_QUARTER_NOTE: u32 = 96;
    
    /// Default timebase for MIDI files
    pub const DEFAULT_TIMEBASE: u16 = 96;
}

/// MIDI event timing offsets
pub mod event_timing {
    /// Program change event timing offset
    pub const PROGRAM_CHANGE_OFFSET: u32 = 192;
    
    /// Pan control event timing offset
    pub const PAN_CONTROL_OFFSET: u32 = 193;
    
    /// Reverb control event timing offset
    pub const REVERB_CONTROL_OFFSET: u32 = 194;
    
    /// Default track start time
    pub const TRACK_START_TIME: u32 = 384;
    
    /// Empty track end time
    pub const EMPTY_TRACK_END_TIME: u32 = 385;
}

/// MML parsing constants
pub mod mml {
    /// Minimum volume value
    pub const MIN_VOLUME: u8 = 1;
    
    /// Maximum volume value
    pub const MAX_VOLUME: u8 = 15;
    
    /// Maximum octave value
    pub const MAX_OCTAVE: u8 = 9;
    
    /// Default volume
    pub const DEFAULT_VOLUME: i32 = 8;
    
    /// Default octave
    pub const DEFAULT_OCTAVE: i32 = 4;
    
    /// MIDI note offset
    pub const MIDI_NOTE_OFFSET: i32 = 12;
    
    /// Velocity multiplier for volume
    pub const VELOCITY_MULTIPLIER: i32 = 8;
}

/// MIDI format constants
pub mod midi {
    /// MIDI header chunk identifier
    pub const HEADER_CHUNK: &str = "MThd";
    
    /// MIDI track chunk identifier
    pub const TRACK_CHUNK: &str = "MTrk";
    
    /// MIDI format type (1 = multiple tracks)
    pub const FORMAT_TYPE: u16 = 1;
    
    /// Tracks per MML (3 tracks per MML string)
    pub const TRACKS_PER_MML: u16 = 3;
    
    /// Default tempo in microseconds (500000 = 120 BPM)
    pub const DEFAULT_TEMPO_MICROSECONDS: u32 = 500000;
}

/// System Exclusive message data
pub mod sysex {
    /// Default SysEx data for Yokoso Project
    pub const YOKOSO_SYSEX_DATA: &[u8] = &[0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7f, 0x00, 0x41, 0xf7];
    
    /// Meta text for Yokoso Project
    pub const YOKOSO_META_TEXT: &str = "Yokoso Project(https://yoko.so/)";
}

/// MIDI control change numbers
pub mod control_change {
    /// Pan control change number
    pub const PAN: u8 = 10;
    
    /// Reverb control change number
    pub const REVERB: u8 = 91;
}
