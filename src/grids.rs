use std::{cmp, collections::HashMap};

use bevy::prelude::{Vec2, Vec3};
use kd_tree::KdTree;

use crate::{
    utils::{calc_weighted_midpoint, window_to_grid},
    H, MAX_PHEROMONE_STRENGTH, PH_CACHE_GRID_SIZE, PH_DECAY_RATE, PH_GRID_OPACITY,
    PH_GRID_VIZ_MIN_STRENGTH, PH_UNIT_GRID_SIZE, W,
};

pub struct WorldGrid {
    pub color: (u8, u8, u8),
    signals: DecayGrid,
    tree: Option<KdTree<[f32; 2]>>,
    steer_cache: HashMap<(i32, i32), Vec2>,
}

impl WorldGrid {
    pub fn new(color: (u8, u8, u8), signals: HashMap<(i32, i32), f32>) -> Self {
        Self {
            color,
            signals: DecayGrid::new(signals, MAX_PHEROMONE_STRENGTH),
            tree: None,
            steer_cache: HashMap::new(),
        }
    }

    pub fn emit_signal(&mut self, key: &(i32, i32), value: f32) {
        let key = &self.get_ph_key(key.0, key.1);
        if key.0 == 0 && key.1 == 0 {
            return;
        }
        self.signals.add_value(key, value, value * 0.25)
    }

    fn get_ph_key(&self, x: i32, y: i32) -> (i32, i32) {
        (x / PH_UNIT_GRID_SIZE as i32, y / PH_UNIT_GRID_SIZE as i32)
    }

    ///获取转向目标
    pub fn get_steer_target(&mut self, pos: &Vec3, radius: f32) -> Option<Vec2> {
        let (x, y) = (pos.x as i32, pos.y as i32);
        let grid_pos = self.get_cache_grid_pos(x, y);
        if let Some(v) = self.steer_cache.get(&grid_pos) {
            return Some(*v);
        }
        match self.get_ph_in_range(pos, radius) {
            Some(v) => {
                if v.is_empty() {
                    return None;
                }

                let steer_target = calc_weighted_midpoint(&v);
                self.steer_cache.insert(grid_pos, steer_target);
                Some(steer_target)
            }
            None => None,
        }
    }

    fn get_pos_from_ph(&self, x: i32, y: i32) -> (i32, i32) {
        (x * PH_UNIT_GRID_SIZE as i32, y * PH_UNIT_GRID_SIZE as i32)
    }

    //取一个模糊点的来提取信号素？
    fn get_cache_grid_pos(&self, x: i32, y: i32) -> (i32, i32) {
        let (tx, ty) = (x + (W as usize / 2) as i32, (H as usize / 2) as i32 - y);
        let (tx, ty) = (tx / PH_CACHE_GRID_SIZE, ty / PH_CACHE_GRID_SIZE);

        (tx, ty)
    }

    //以pos为中心点，取半径为radius范围内的信息素
    fn get_ph_in_range(&self, pos: &Vec3, radius: f32) -> Option<Vec<(i32, i32, f32)>> {
        let key = self.get_ph_key(pos.x as i32, pos.y as i32);
        if let Some(t) = &self.tree {
            let mut ph_items = Vec::new();
            let found = t.within_radius(&[key.0 as f32, key.1 as f32], radius);
            for i in found.iter() {
                let [x, y] = *i;
                let (x, y) = (*x as i32, *y as i32);
                if let Some(v) = self.signals.values.get(&(x, y)) {
                    let world_xy = self.get_pos_from_ph(x, y);
                    ph_items.push((world_xy.0, world_xy.1, *v));
                }
            }
            Some(ph_items)
        } else {
            None
        }
    }

    pub fn update_tree(&mut self) {
        let mut pts = Vec::new();
        for (k, &v) in self.signals.values.iter() {
            if v <= 0.0 {
                continue;
            }

            let (x, y) = *k;
            pts.push([x as f32, y as f32]);
        }
        self.tree = Some(KdTree::build_by_ordered_float(pts));
    }

    pub fn decay_signals(&mut self) {
        self.signals.decay_values(PH_DECAY_RATE);
    }

    pub fn drop_zero_signals(&mut self) {
        self.signals.drop_zero_values();
    }

    pub fn get_signals(&self) -> &HashMap<(i32, i32), f32> {
        self.signals.get_values()
    }

    pub fn clear_steer_cache(&mut self) -> u32 {
        let ret = self.steer_cache.len();
        self.steer_cache.clear();

        ret as u32
    }
}

pub struct DecayGrid {
    max_allowed_value: f32,
    values: HashMap<(i32, i32), f32>,
}

impl DecayGrid {
    pub fn new(values: HashMap<(i32, i32), f32>, max_allowed_value: f32) -> Self {
        Self {
            values,
            max_allowed_value,
        }
    }

    pub fn add_value(&mut self, key: &(i32, i32), value: f32, increment_value: f32) {
        if value <= 0.0 {
            return;
        }

        match self.values.get_mut(key) {
            Some(old_value) => {
                *old_value = (increment_value + *old_value).min(self.max_allowed_value);
            }
            None => {
                self.values.insert(*key, value);
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

pub fn add_map_to_grid_img(
    map: &HashMap<(i32, i32), f32>,
    color: &(u8, u8, u8),
    img_bytes: &mut Vec<u8>,
    use_grid_pos: bool,
) {
    let w = W as usize / PH_UNIT_GRID_SIZE;
    for (k, v) in map.iter() {
        let (x, y) = if use_grid_pos {
            let (x, y) = (
                k.0 * PH_UNIT_GRID_SIZE as i32,
                k.1 * PH_UNIT_GRID_SIZE as i32,
            );
            window_to_grid(x, y)
        } else {
            (k.0, k.1)
        };

        let idx = y * w as i32 + x;
        let strength = cmp::min((*v as u32).saturating_mul(5), u8::MAX.into()) as u8;

        let idx = (idx as usize).saturating_mul(4);
        if idx.saturating_add(3) >= img_bytes.len() || strength < PH_GRID_VIZ_MIN_STRENGTH {
            continue;
        }

        img_bytes[idx + 3] = cmp::min(img_bytes[idx + 3].saturating_add(strength), PH_GRID_OPACITY);
        img_bytes[idx] = color.0;
        img_bytes[idx + 1] = color.1;
        img_bytes[idx + 2] = color.2;
    }
}
