use kurbo::{Circle, Rect};

use crate::{
    particle::GeoQuery,
    v2::{TreeValue, V2},
};

pub enum QuadTreeNode<T> {
    Empty,
    Leaf { value: T },
    Node(Box<[QuadTree<T>; 4]>),
}

pub struct QuadTree<T> {
    node: QuadTreeNode<T>,
    center: V2,
    half_width: f64,
    half_height: f64,
    circ: Circle,
}

pub enum Quadrant {
    NW,
    NE,
    SW,
    SE,
}

impl<T> QuadTreeNode<T> {
    fn take(&mut self) -> QuadTreeNode<T> {
        std::mem::replace(self, QuadTreeNode::Empty)
    }
}

impl<T: TreeValue> QuadTree<T> {
    pub fn new(center: V2, half_width: f64, half_height: f64) -> QuadTree<T> {
        QuadTree {
            circ: Circle::new((center.x, center.y), half_width * 2.0_f64.sqrt()),
            node: QuadTreeNode::Empty,
            center,
            half_width,
            half_height,
        }
    }

    fn add_vec(&mut self, vec: Vec<T>) {
        vec.into_iter().for_each(|v| {
            self.insert(v);
        });
    }

    pub fn get_circ(&self) -> Circle {
        self.circ
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.center.x - self.half_width,
            self.center.y - self.half_height,
            self.half_width * 2.,
            self.half_height * 2.,
        )
    }

    pub fn for_each(&self, f: impl Fn(&QuadTree<T>)) {
        self._for_each(&f);
    }

    fn _for_each(&self, f: &impl Fn(&QuadTree<T>)) {
        f(self);
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value: _ } => {}
            QuadTreeNode::Node(v) => {
                let nw = &v[0];
                let ne = &v[1];
                let sw = &v[2];
                let se = &v[3];
                nw._for_each(f);
                ne._for_each(f);
                sw._for_each(f);
                se._for_each(f);
            }
        }
    }

    pub fn insert(&mut self, mut value: T) {
        let node = self.node.take();
        self.node = match node {
            QuadTreeNode::Empty => QuadTreeNode::Leaf { value },
            QuadTreeNode::Leaf { value: this_value } => {
                let mut other =
                    QuadTree::new_node(self.center.clone(), self.half_width, self.half_height);
                if value.position().sub(&this_value.position()).len() < 0.001 {
                    value.offset_pos();
                    return;
                }
                other.insert(this_value);
                other.insert(value);
                *self = other;
                return;
            }
            QuadTreeNode::Node(mut v) => {
                let quadrant = self.quadrant(&value.position());
                match quadrant {
                    Quadrant::NW => v[0].insert(value),
                    Quadrant::NE => v[1].insert(value),
                    Quadrant::SW => v[2].insert(value),
                    Quadrant::SE => v[3].insert(value),
                };
                QuadTreeNode::Node(v)
            }
        }
    }

    pub fn new_node(center: V2, half_width: f64, half_height: f64) -> QuadTree<T> {
        let nw = QuadTree::new(
            center.sub(&V2::new(half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let ne = QuadTree::new(
            center.add(&V2::new(half_width / 2., -half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let sw = QuadTree::new(
            center.add(&V2::new(-half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        let se = QuadTree::new(
            center.add(&V2::new(half_width / 2., half_height / 2.)),
            half_width / 2.,
            half_height / 2.,
        );
        QuadTree {
            node: QuadTreeNode::Node(Box::new([nw, ne, sw, se])),
            circ: Circle::new((center.x, center.y), half_width * 2.0_f64.sqrt()),
            center,
            half_width,
            half_height,
        }
    }

    fn quadrant(&self, point: &V2) -> Quadrant {
        let center = &self.center;
        if point.x < center.x {
            if point.y < center.y {
                Quadrant::NW
            } else {
                Quadrant::SW
            }
        } else {
            if point.y < center.y {
                Quadrant::NE
            } else {
                Quadrant::SE
            }
        }
    }

    pub fn query_distance_path(&self, point: &V2, r: f64) -> Vec<&QuadTree<T>> {
        let mut vec = Vec::new();
        let circle = Circle::new((point.x, point.y), r);
        if !circles_intersect(&self.circ, &circle) {
            return vec;
        } else {
        }
        vec.push(self);
        // rect.bounding_box()
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { .. } => {}
            QuadTreeNode::Node(arr) => {
                vec.extend(arr[0].query_distance_path(point, r));
                vec.extend(arr[1].query_distance_path(point, r));
                vec.extend(arr[2].query_distance_path(point, r));
                vec.extend(arr[3].query_distance_path(point, r));
            }
        }
        vec
    }

    fn _query_distance(&self, r: &Circle, f: &mut impl FnMut(&T)) {
        if !circles_intersect(&self.circ, r) {
            return;
        }
        // rect.bounding_box()
        match &self.node {
            QuadTreeNode::Empty => {}
            QuadTreeNode::Leaf { value } => f(value),
            QuadTreeNode::Node(arr) => {
                arr[0]._query_distance(r, f);
                arr[1]._query_distance(r, f);
                arr[2]._query_distance(r, f);
                arr[3]._query_distance(r, f);
            }
        }
    }
}

impl<T: TreeValue> GeoQuery<T> for QuadTree<T> {
    fn query_distance(&self, point: &V2, r: f64, mut f: impl FnMut(&T)) {
        let circ = Circle::new((point.x, point.y), r);
        self._query_distance(&circ, &mut f);
    }

    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self {
        let mut tree = QuadTree::new(V2::new(0., 0.), max_dim, max_dim);
        //print tree circle
        tree.add_vec(vec);
        tree
    }
}

fn circles_intersect(a: &Circle, b: &Circle) -> bool {
    let r = a.radius + b.radius;
    let dx = a.center.x - b.center.x;
    let dy = a.center.y - b.center.y;
    r * r > dx * dx + dy * dy
}

#[cfg(test)]
mod tests {
    use super::*;

    impl TreeValue for V2 {
        fn position(&self) -> V2 {
            self.clone()
        }
        fn offset_pos(&mut self) {
            *self = self.add(&V2::new(0.0001, 0.0001));
        }
    }

    #[test]
    fn test_insert() {
        // let mut tree = QuadTree::new(V2::new(0., 0.), 1., 1.);
        // tree.insert(V2::new(0.5, 0.5));
        // tree.insert(V2::new(0.25, 0.25));
        // tree.insert(V2::new(0.75, 0.75));
        // tree.insert(V2::new(0.125, 0.125));

        // let v = tree.query_distance(&V2::new(0.5, 0.5), 1.0);
        // assert_eq!(v.len(), 4)
    }
}
