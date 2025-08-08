use crate::track_event::*;

pub struct TrackBuilder {
    channel: u8,
    events: Vec<Box<dyn TrackEvent>>,
}

impl TrackBuilder {
    pub fn new(channel: u8) -> Self {
        TrackBuilder {
            channel,
            events: Vec::new(),
        }
    }

    pub fn put_event(&mut self, event: Box<dyn TrackEvent>) -> &mut Self {
        self.events.push(event);
        self
    }

    pub fn put_events(&mut self, events: Vec<Box<dyn TrackEvent>>) -> &mut Self {
        self.events.extend(events);
        self
    }

    pub fn build(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.push(format!("MTrk"));
        
        for event in &self.events {
            result.push(format!("{} {}", event.lead_time(), event.value()));
        }
        
        result
    }

    pub fn event_list(&self) -> &[Box<dyn TrackEvent>] {
        &self.events
    }
}