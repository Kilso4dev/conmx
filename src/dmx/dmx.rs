use std::collections::HashMap;

use super::dmx_universe::DMXUniverse;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct DMX {
    universes: HashMap<usize, DMXUniverse>,
}

impl DMX {
    pub fn new() -> Self {
        DMX {
            universes: HashMap::new(),
        }
    }

    pub fn add_universe(&mut self, univ: DMXUniverse) -> &mut Self {
        self.universes.insert(univ.get_id(), univ);
        self
    }
    pub fn get_universe(&mut self, id: usize) -> Option<&mut DMXUniverse> {
        self.universes.get_mut(&id)
    }
}
