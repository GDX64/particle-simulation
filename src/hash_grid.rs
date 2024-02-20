use std::{
    collections::HashMap,
    hash::{BuildHasher, Hasher},
};

use kurbo::Rect;

use crate::{v2::TreeValue, GeoQuery};

struct FastHasher {
    value: u64,
}
struct FastHasherBuilder {}

impl Hasher for FastHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.value ^= (byte as u64) << (i % 8 * 8);
        }
    }
}

impl BuildHasher for FastHasherBuilder {
    type Hasher = FastHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FastHasher { value: 0 }
    }
}

pub struct HashGrid<T> {
    max_dim: f64,
    divisor: f64,
    data: HashMap<(i32, i32), Vec<T>, FastHasherBuilder>,
}

impl<T: TreeValue> HashGrid<T> {
    fn _from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        let divisor = 10.;
        let mut grid = HashGrid {
            max_dim,
            data: HashMap::with_hasher(FastHasherBuilder {}),
            divisor,
        };
        for value in vec {
            let key = grid.calc_cell(&value.position());
            grid.data.entry(key).or_insert(vec![]).push(value);
        }
        grid
    }

    fn calc_cell(&self, point: &crate::v2::V2) -> (i32, i32) {
        let x = (point.x / self.divisor).floor();
        let y = (point.y / self.divisor).floor();
        (x as i32, y as i32)
    }

    pub fn get_rects(&self) -> Vec<Rect> {
        let mut rects = vec![];
        for (key, _) in &self.data {
            let x = key.0 as f64 * self.divisor;
            let y = key.1 as f64 * self.divisor;
            let width = self.divisor;
            let height = self.divisor;
            rects.push(Rect::new(x, y, x + width, y + height));
        }
        rects
    }

    fn neighbor_keys(&self, key: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        let (x, y) = key.clone();
        (0..9).map(move |i| {
            let x = x + i / 3 - 1;
            let y = y + i % 3 - 1;
            (x, y)
        })
    }
}

impl<T: TreeValue> GeoQuery<T> for HashGrid<T> {
    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        Self::_from_vec(vec, max_dim)
    }

    #[inline(never)]
    fn query_distance(&self, point: &crate::v2::V2, radius: f64, mut f: impl FnMut(&T)) {
        let key = self.calc_cell(point);
        self.neighbor_keys(&key).for_each(|key| {
            if let Some(values) = self.data.get(&key) {
                for value in values {
                    if value.position().distance_to(point) < radius {
                        f(value);
                    }
                }
            }
        });
    }
}
