use std::collections::HashMap;

pub struct WorldGrid {
    pub color: (u8, u8, u8),
    signals: DecayGrid,
}

impl WorldGrid {
    pub fn emit_signal(&mut self, key: &(i32, i32), value: f32) {
        self.signals.add_value(&key, value, value * 0.25)
    }
}

pub struct DecayGrid {
    max_allowed_value: f32,
    values: HashMap<(i32, i32), f32>,
}

impl DecayGrid {
    pub fn add_value(&mut self, key: &(i32, i32), value: f32, increment_value: f32) {
        if value <= 0.0 {
            return;
        }

        match self.values.get_mut(key) {
            Some(old_value) => {
                *old_value = (increment_value + *old_value).min(self.max_allowed_value);
            }
            None => {
                self.values.insert(key.clone(), value);
            }
        }
    }

    pub fn decay_values(&mut self, decay_rate: f32) {
        for (_, v) in self.values.iter_mut() {
            *v = f32::max(*v - decay_rate, 0.0);
        }
    }

    pub fn drop_zero_values(&mut self) {
        self.values.retain(|_, v| *v > 0.0);
    }

    pub fn get_values(&self) -> &HashMap<(i32, i32), f32> {
        &self.values
    }
}
