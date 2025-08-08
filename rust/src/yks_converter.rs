use crate::byte_buffer::ByteBuffer;
use crate::mf2tt2mf::Mf2tt2mf;

const START_TIMEBASE: u16 = 96;

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

    pub fn to_buffer(&self) -> Option<ByteBuffer> {
        if self.mml.len() != self.inst.len() {
            eprintln!("MML 갯수와 익기 갯수가 다릅니다.");
            return None;
        }

        let mut byte_buffer = ByteBuffer::new();

        // Header Chunk
        let default_buffer = [0x00, 0x00, 0x00, 0x06, 0x00];
        byte_buffer.put_string("MThd");
        byte_buffer.put_bytes_array(&default_buffer);
        
        // format: 1 (multiple tracks)
        byte_buffer.put_byte(1);
        
        // track count (2 bytes)
        byte_buffer.put_u16((self.mml.len() * 3) as u16);
        
        // timebase (2 bytes)  
        byte_buffer.put_u16(self.timebase);

        for (i, mml) in self.mml.iter().enumerate() {
            let mut mf2tt2mf = Mf2tt2mf::new((i + 1) as u8, self.inst[i], 64, 0);
            
            if !mf2tt2mf.from_mml(mml) {
                return None;
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
                        eprintln!("Event Convert error: {}", event.value());
                        continue;
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

                byte_buffer.put_string("MTrk");
                let track_length = track_buffer.size() as u32;
                byte_buffer.put_u32(track_length);
                byte_buffer.put_bytes(&track_buffer);
            }
        }

        Some(byte_buffer)
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