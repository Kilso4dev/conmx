#[derive(Debug, PartialEq, Clone, Default)]
pub struct DMXChannel {
    value: u32,
    or_value: u32,
    or: bool,
}

impl DMXChannel {
    pub fn new() -> Self { DMXChannel::default() }

    pub fn override_val(&mut self, value: u32) -> &mut Self {
        self.or = true;
        self.or_value = value;
        self
    }
    pub fn revert_override(&mut self) -> &mut Self { self.or = false; self }

    pub fn set_val(&mut self, val: u32) -> &mut Self { self.value = val; self }

    pub fn get_val(&mut self) -> u32 {
        if self.or { self.or_value } else { self.value }
    }
}
