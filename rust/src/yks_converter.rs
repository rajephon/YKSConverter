use crate::byte_buffer::ByteBuffer;
use crate::mf2tt2mf::Mf2tt2mf;
use crate::errors::ConversionError;
use crate::constants::{timing, midi};

const START_TIMEBASE: u16 = timing::DEFAULT_TIMEBASE;

pub struct YksConverter {
    mml: Vec<String>,
    inst: Vec<u8>,
    timebase: u16,
}

impl YksConverter {
    pub fn new(mml: String, inst: u8) -> Self {
        YksConverter {
            mml: vec![mml],
            inst: vec![inst],
            timebase: START_TIMEBASE,
        }
    }

    pub fn new_multi(mml: Vec<String>, inst: Vec<u8>) -> Self {
        YksConverter {
            mml,
            inst,
            timebase: START_TIMEBASE,
        }
    }

    pub fn set_mml(&mut self, mml: String) {
        self.mml = vec![mml];
    }

    pub fn set_mml_multi(&mut self, mml: Vec<String>) {
        self.mml = mml;
    }

    pub fn set_inst(&mut self, inst: u8) {
        self.inst = vec![inst];
    }

    pub fn set_inst_multi(&mut self, inst: Vec<u8>) {
        self.inst = inst;
    }

    pub fn mml(&self) -> &[String] {
        &self.mml
    }

    pub fn inst(&self) -> &[u8] {
        &self.inst
    }

    /// Converts MML to MIDI buffer
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(ByteBuffer)` on success, or `Err(ConversionError)` on failure.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use yks_converter::YksConverter;
    /// 
    /// let converter = YksConverter::new("MML@c,,;".to_string(), 1);
    /// let buffer = converter.to_buffer_result().unwrap();
    /// ```
    pub fn to_buffer_result(&self) -> Result<ByteBuffer, ConversionError> {
        if self.mml.len() != self.inst.len() {
            return Err(ConversionError::MmlInstCountMismatch {
                mml_count: self.mml.len(),
                inst_count: self.inst.len(),
            });
        }

        let mut byte_buffer = ByteBuffer::new();

        // Header Chunk
        let default_buffer = [0x00, 0x00, 0x00, 0x06, 0x00];
        byte_buffer.put_string(midi::HEADER_CHUNK);
        byte_buffer.put_bytes_array(&default_buffer);
        
        // format: 1 (multiple tracks)
        byte_buffer.put_byte(midi::FORMAT_TYPE as u8);
        
        // track count (2 bytes)
        byte_buffer.put_u16((self.mml.len() as u16) * midi::TRACKS_PER_MML);
        
        // timebase (2 bytes)  
        byte_buffer.put_u16(self.timebase);

        for (i, mml) in self.mml.iter().enumerate() {
            let mut mf2tt2mf = Mf2tt2mf::new((i + 1) as u8, self.inst[i], 64, 0);
            
            if !mf2tt2mf.from_mml(mml) {
                return Err(ConversionError::MmlParseFailed(mml.clone()));
            }

            let track_event_list = mf2tt2mf.build();

            for event_list in track_event_list {
                let mut time = 0u32;
                let mut last = 0x00u8;
                let mut track_buffer = ByteBuffer::new();

                for event in event_list {
                    let delta_time = event.lead_time() - time;
                    time = event.lead_time();
                    
                    let var_len_buffer = write_var_len(delta_time);
                    track_buffer.put_bytes(&var_len_buffer);

                    let event_buffer = event.to_buffer();
                    if event_buffer.size() <= 0 {
                        return Err(ConversionError::EventConversionFailed(event.value()));
                    }

                    let start = event_buffer.get_at(0);
                    if start < 0x80 || start > 0xef || start != last {
                        track_buffer.put_byte(start);
                    }
                    
                    for i in 1..event_buffer.size() {
                        track_buffer.put_byte(event_buffer.get_at(i));
                    }
                    last = start;
                }

                byte_buffer.put_string(midi::TRACK_CHUNK);
                let track_length = track_buffer.size() as u32;
                byte_buffer.put_u32(track_length);
                byte_buffer.put_bytes(&track_buffer);
            }
        }

        Ok(byte_buffer)
    }

    /// Legacy method for backward compatibility
    /// 
    /// This method maintains the original API for existing code.
    /// For new code, prefer using `to_buffer_result()` for better error handling.
    pub fn to_buffer(&self) -> Option<ByteBuffer> {
        match self.to_buffer_result() {
            Ok(buffer) => Some(buffer),
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        }
    }
}

fn write_var_len(mut value: u32) -> ByteBuffer {
    let mut buf = (value & 0x7f) as u32;
    let mut byte_buffer = ByteBuffer::new();

    value >>= 7;
    while value > 0 {
        buf <<= 8;
        buf |= (value & 0x7f) | 0x80;
        value >>= 7;
    }

    loop {
        byte_buffer.put_byte((buf % 256) as u8);
        if (buf & 0x80) != 0 {
            buf >>= 8;
        } else {
            break;
        }
    }

    byte_buffer
}
