use super::{particle::GeoQuery, v2::TreeValue, v2::V2};
use kurbo::{Circle, Rect, Shape};

struct OrderStore<T> {
    value: T,
    order: u64,
}

pub trait SpaceFillingCurve {
    type T: TreeValue;
    fn number_of(x: f64, y: f64) -> u64;
    fn order_of(v: &Self::T) -> u64 {
        Self::number_of(v.x(), v.y())
    }
    fn pair_of(order: u64) -> (f64, f64);
}

pub struct SpaceFillingTree<S: SpaceFillingCurve> {
    values: Vec<OrderStore<S::T>>,
}

impl<S: SpaceFillingCurve> SpaceFillingTree<S> {
    pub fn from_vec(vec: Vec<S::T>) -> Self {
        let mut tree = SpaceFillingTree { values: Vec::new() };
        let mut v = vec
            .into_iter()
            .map(|value| {
                let order = S::order_of(&value);
                OrderStore { value, order }
            })
            .collect::<Vec<OrderStore<S::T>>>();
        v.sort_by_key(|v| v.order);
        tree.values = v;
        tree
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a S::T> {
        self.values.iter().map(|v| &v.value)
    }

    pub fn get_rect_limits(&self, rect: &Rect) -> (u64, u64) {
        let top_left = S::number_of(rect.x0, rect.y0);
        let top_right = S::number_of(rect.x1, rect.y0);
        let bottom_left = S::number_of(rect.x0, rect.y1);
        let bottom_right = S::number_of(rect.x1, rect.y1);
        let max_value = u64::max(
            u64::max(top_left, top_right),
            u64::max(bottom_left, bottom_right),
        );
        let min_value = u64::min(
            u64::min(top_left, top_right),
            u64::min(bottom_left, bottom_right),
        );
        (min_value, max_value)
    }

    fn query_rect(&self, rect: &Rect) -> &[OrderStore<S::T>] {
        let (min_value, max_value) = self.get_rect_limits(rect);
        let start = self.find_order_index(min_value);
        let end = self.find_order_index(max_value);
        let r = self.values.get(start..end);
        r.unwrap_or(&[])
    }

    pub fn number_of(&self, x: f64, y: f64) -> u64 {
        S::number_of(x, y)
    }

    pub fn pair_of(&self, order: u64) -> (f64, f64) {
        S::pair_of(order)
    }

    fn find_order_index(&self, order: u64) -> usize {
        let r = self.values.binary_search_by(|value| {
            if value.order == order {
                return std::cmp::Ordering::Equal;
            }
            if value.order < order {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
        match r {
            Ok(i) => i,
            Err(i) => i,
        }
    }
}

impl<S: SpaceFillingCurve> GeoQuery<S::T> for SpaceFillingTree<S> {
    fn query_distance(&self, point: &V2, radius: f64, mut f: impl FnMut(&S::T)) {
        let rect = Circle::new((point.x, point.y), radius).bounding_box();
        let slice = self.query_rect(&rect);
        slice.iter().for_each(|value| f(&value.value));
    }

    fn from_vec(vec: Vec<S::T>, max_dim: f64) -> Self {
        SpaceFillingTree::from_vec(vec)
    }
}

pub mod curves {
    use super::super::v2::TreeValue;

    pub struct HilbertCurve<T> {
        _t: std::marker::PhantomData<T>,
    }

    const FILL_SCALE: f64 = 20.;
    impl<T: TreeValue> super::SpaceFillingCurve for HilbertCurve<T> {
        type T = T;
        fn number_of(x: f64, y: f64) -> u64 {
            let x = x / FILL_SCALE;
            let y = y / FILL_SCALE;
            fast_hilbert::xy2h(x as u64, y as u64, 8) as u64
        }
        fn pair_of(order: u64) -> (f64, f64) {
            let (x, y) = fast_hilbert::h2xy::<u32>(order, 8);
            (x as f64 * FILL_SCALE, y as f64 * FILL_SCALE)
        }
    }

    pub struct ZOrderCurve<T> {
        _t: std::marker::PhantomData<T>,
    }

    fn z_order(x: u64, y: u64, order: u64) -> u64 {
        let mut z = 0;
        for i in 0..order {
            z |= ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1));
        }
        z
    }
    impl<T: TreeValue> super::SpaceFillingCurve for ZOrderCurve<T> {
        type T = T;

        fn number_of(x: f64, y: f64) -> u64 {
            z_order((x / FILL_SCALE) as u64, (y / FILL_SCALE) as u64, 16)
        }

        fn pair_of(morton: u64) -> (f64, f64) {
            let order = 32;
            let mut x = 0;
            let mut y = 0;
            for i in 0..order {
                let mask = 1 << i;
                if morton & (mask << i) != 0 {
                    x |= mask;
                }
                if morton & (mask << (i + 1)) != 0 {
                    y |= mask;
                }
            }
            (x as f64 * FILL_SCALE, y as f64 * FILL_SCALE)
        }
    }

    #[test]
    fn z_order_limits() {
        let result = z_order(2, 1, 32);
        println!("z_order: {}", result);
    }
}
