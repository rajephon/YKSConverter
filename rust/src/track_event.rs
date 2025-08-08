use crate::byte_buffer::ByteBuffer;

pub trait TrackEvent {
    fn lead_time(&self) -> u32;
    fn set_lead_time(&mut self, time: u32);
    fn value(&self) -> String;
    fn to_buffer(&self) -> ByteBuffer;
    fn clone_event(&self) -> Box<dyn TrackEvent>;
}

#[derive(Debug, Clone)]
pub struct SeqSpec {
    time: u32,
    value: Vec<u8>,
}

impl SeqSpec {
    pub fn new(value: Vec<u8>) -> Self {
        SeqSpec { time: 0, value }
    }
}

impl TrackEvent for SeqSpec {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        self.value.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_bytes_array(&self.value);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(SeqSpec::new(self.value.clone()));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct MetaText {
    time: u32,
    text: String,
}

impl MetaText {
    pub fn new(text: String) -> Self {
        MetaText { time: 0, text }
    }
}

impl TrackEvent for MetaText {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("Text: {}", self.text)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xFF);
        buffer.put_byte(0x01);
        buffer.put_byte(self.text.len() as u8);
        buffer.put_string(&self.text);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(MetaText::new(self.text.clone()));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct Tempo {
    time: u32,
    tempo: u32,
}

impl Tempo {
    pub fn new(tempo: u32) -> Self {
        Tempo { time: 0, tempo }
    }
}

impl TrackEvent for Tempo {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("Tempo: {}", self.tempo)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xFF);
        buffer.put_byte(0x51);
        buffer.put_byte(0x03);
        buffer.put_byte((self.tempo >> 16) as u8);
        buffer.put_byte((self.tempo >> 8) as u8);
        buffer.put_byte(self.tempo as u8);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(Tempo::new(self.tempo));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct SysEx {
    time: u32,
    data: Vec<u8>,
}

impl SysEx {
    pub fn new(data: Vec<u8>) -> Self {
        SysEx { time: 0, data }
    }
}

impl TrackEvent for SysEx {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("SysEx: {}", self.data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "))
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xF0);
        buffer.put_byte(self.data.len() as u8);
        buffer.put_bytes_array(&self.data);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(SysEx::new(self.data.clone()));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct ProgramChange {
    time: u32,
    channel: u8,
    program: u8,
}

impl ProgramChange {
    pub fn new(channel: u8, program: u8) -> Self {
        ProgramChange { time: 0, channel, program }
    }
}

impl TrackEvent for ProgramChange {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("ProgramChange: ch={}, program={}", self.channel, self.program)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xC0 + (self.channel - 1));
        buffer.put_byte(self.program);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(ProgramChange::new(self.channel, self.program));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct ControlChange {
    time: u32,
    channel: u8,
    controller: u8,
    value: u8,
}

impl ControlChange {
    pub fn new(channel: u8, controller: u8, value: u8) -> Self {
        ControlChange { time: 0, channel, controller, value }
    }
}

impl TrackEvent for ControlChange {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("ControlChange: ch={}, cc={}, val={}", self.channel, self.controller, self.value)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xB0 + (self.channel - 1));
        buffer.put_byte(self.controller);
        buffer.put_byte(self.value);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(ControlChange::new(self.channel, self.controller, self.value));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct NoteOn {
    time: u32,
    channel: u8,
    note: u8,
    velocity: u8,
}

impl NoteOn {
    pub fn new(channel: u8, note: u8, velocity: u8) -> Self {
        NoteOn { time: 0, channel, note, velocity }
    }
}

impl TrackEvent for NoteOn {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("NoteOn: ch={}, note={}, vel={}", self.channel, self.note, self.velocity)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0x90 + (self.channel - 1));
        buffer.put_byte(self.note);
        buffer.put_byte(self.velocity);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(NoteOn::new(self.channel, self.note, self.velocity));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct NoteOff {
    time: u32,
    channel: u8,
    note: u8,
    velocity: u8,
}

impl NoteOff {
    pub fn new(channel: u8, note: u8, velocity: u8) -> Self {
        NoteOff { time: 0, channel, note, velocity }
    }
    
    pub fn note(&self) -> u8 { self.note }
}

impl TrackEvent for NoteOff {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        format!("NoteOff: ch={}, note={}, vel={}", self.channel, self.note, self.velocity)
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0x80 + (self.channel - 1));
        buffer.put_byte(self.note);
        buffer.put_byte(self.velocity);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(NoteOff::new(self.channel, self.note, self.velocity));
        clone.set_lead_time(self.time);
        clone
    }
}

#[derive(Debug, Clone)]
pub struct EndOfTrack {
    time: u32,
}

impl EndOfTrack {
    pub fn new() -> Self {
        EndOfTrack { time: 0 }
    }
}

impl TrackEvent for EndOfTrack {
    fn lead_time(&self) -> u32 { self.time }
    fn set_lead_time(&mut self, time: u32) { self.time = time; }
    
    fn value(&self) -> String {
        "EndOfTrack".to_string()
    }
    
    fn to_buffer(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_byte(0xFF);
        buffer.put_byte(0x2F);
        buffer.put_byte(0x00);
        buffer
    }

    fn clone_event(&self) -> Box<dyn TrackEvent> {
        let mut clone = Box::new(EndOfTrack::new());
        clone.set_lead_time(self.time);
        clone
    }
}

impl Default for EndOfTrack {
    fn default() -> Self {
        Self::new()
    }
}