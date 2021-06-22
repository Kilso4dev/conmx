use log::{ warn };

use super::dmx_channel::DMXChannel;

#[derive(Debug, Clone)]
pub struct DMXUniverse {
    id: usize,
    channels: Vec<DMXChannel>,
}

impl DMXUniverse {
    pub fn new(id: usize) -> Self {
        DMXUniverse {
            id,
            channels: vec![DMXChannel::new(); 512],
        }
    }

    pub fn set_channel(&mut self, id: usize, val: u32) -> &Self {
        match self.channels.get_mut(id) {
            Some(channel) => { channel.set_val(val); },
            None => warn!("Trying to write outside of range (Index: {})", id),
        }
        self
    }

    pub fn set_override_channel(&mut self, id: usize, val: u32) -> &Self {
        match self.channels.get_mut(id) {
            Some(channel) => { channel.override_val(val); },
            None => warn!("Trying to override channel outside of range (Index: {})", id),
        }
        self
    }

    pub fn get_channel(&self, id: usize) -> Option<&DMXChannel> { self.channels.get(id) }

    pub fn get_id(&self) -> usize { self.id }
}

impl PartialEq for DMXUniverse {
    fn eq(&self, other: &DMXUniverse) -> bool { self.id == other.id }
}
