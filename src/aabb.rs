#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aabb {
    pub min_x: i64,
    pub min_y: i64,
    pub max_x: i64,
    pub max_y: i64,
}

impl Aabb {
    pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> Self {
        assert!(min_x <= max_x, "AABB: min_x ({min_x}) > max_x ({max_x})");
        assert!(min_y <= max_y, "AABB: min_y ({min_y}) > max_y ({max_y})");
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn from_vertices(xs: &[i64], ys: &[i64]) -> Self {
        assert!(!xs.is_empty(), "AABB requires at least 1 vertex");
        assert_eq!(xs.len(), ys.len(), "xs and ys must have same length");

        let mut min_x = xs[0];
        let mut max_x = xs[0];
        let mut min_y = ys[0];
        let mut max_y = ys[0];

        for (&x, &y) in xs[1..].iter().zip(ys[1..].iter()) {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }

    pub fn contains_point(&self, x: i64, y: i64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }

    pub fn width(&self) -> i64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> i64 {
        self.max_y - self.min_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: i64 = 1_000_000;

    #[test]
    fn from_vertices_tracks_extrema() {
        let xs = vec![M, 2 * M, 2 * M, M];
        let ys = vec![M, M, 2 * M, 2 * M];
        let aabb = Aabb::from_vertices(&xs, &ys);
        assert_eq!(aabb.min_x, M);
        assert_eq!(aabb.max_x, 2 * M);
        assert_eq!(aabb.min_y, M);
        assert_eq!(aabb.max_y, 2 * M);
    }

    #[test]
    fn from_vertices_handles_single_vertex() {
        let xs = vec![5];
        let ys = vec![3];
        let aabb = Aabb::from_vertices(&xs, &ys);

        assert_eq!(aabb.min_x, 5);
        assert_eq!(aabb.max_x, 5);
        assert_eq!(aabb.min_y, 3);
        assert_eq!(aabb.max_y, 3);
    }

    #[test]
    fn from_vertices_handles_negative_coordinates() {
        let xs = vec![-10, 10];
        let ys = vec![-5, 5];
        let aabb = Aabb::from_vertices(&xs, &ys);

        assert_eq!(aabb.min_x, -10);
        assert_eq!(aabb.max_x, 10);
        assert_eq!(aabb.min_y, -5);
        assert_eq!(aabb.max_y, 5);
    }

    #[test]
    fn intersects_overlapping_returns_true() {
        let a = Aabb::new(0, 0, 2 * M, 2 * M);
        let b = Aabb::new(M, M, 3 * M, 3 * M);
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn intersects_touching_edge_returns_false() {
        let a = Aabb::new(0, 0, M, M);
        let b = Aabb::new(M, 0, 2 * M, M);
        assert!(!a.intersects(&b), "touching edges should NOT intersect");
        assert!(
            !b.intersects(&a),
            "touching edges should NOT intersect (reversed)"
        );
    }

    #[test]
    fn intersects_touching_corner_returns_false() {
        let a = Aabb::new(0, 0, M, M);
        let b = Aabb::new(M, M, 2 * M, 2 * M);
        assert!(!a.intersects(&b), "touching corners should NOT intersect");
    }

    #[test]
    fn intersects_separated_returns_false() {
        let a = Aabb::new(0, 0, M, M);
        let b = Aabb::new(2 * M, 0, 3 * M, M);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn intersects_contained_returns_true() {
        let outer = Aabb::new(0, 0, 10 * M, 10 * M);
        let inner = Aabb::new(2 * M, 2 * M, 5 * M, 5 * M);
        assert!(outer.intersects(&inner));
        assert!(inner.intersects(&outer));
    }

    #[test]
    fn contains_point_works() {
        let aabb = Aabb::new(0, 0, M, M);
        assert!(aabb.contains_point(M / 2, M / 2));
        assert!(aabb.contains_point(0, 0));
        assert!(aabb.contains_point(M, M));
        assert!(!aabb.contains_point(M + 1, 0));
    }

    #[test]
    fn merge_covers_both() {
        let a = Aabb::new(0, 0, M, M);
        let b = Aabb::new(2 * M, 2 * M, 3 * M, 3 * M);
        let merged = a.merge(&b);
        assert_eq!(merged.min_x, 0);
        assert_eq!(merged.max_x, 3 * M);
        assert_eq!(merged.min_y, 0);
        assert_eq!(merged.max_y, 3 * M);
    }

    #[test]
    fn merge_disjoint_boxes_contains_both() {
        let a = Aabb::new(-3 * M, -2 * M, -M, -M);
        let b = Aabb::new(2 * M, M, 4 * M, 3 * M);
        let merged = a.merge(&b);

        assert_eq!(merged.min_x, -3 * M);
        assert_eq!(merged.min_y, -2 * M);
        assert_eq!(merged.max_x, 4 * M);
        assert_eq!(merged.max_y, 3 * M);
        assert!(merged.contains_point(a.min_x, a.min_y));
        assert!(merged.contains_point(b.max_x, b.max_y));
    }

    #[test]
    fn merge_nested_boxes_equals_outer_box() {
        let outer = Aabb::new(0, 0, 10 * M, 10 * M);
        let inner = Aabb::new(2 * M, 3 * M, 4 * M, 5 * M);

        assert_eq!(outer.merge(&inner), outer);
        assert_eq!(inner.merge(&outer), outer);
    }

    #[test]
    fn contains_point_includes_corner_and_excludes_outside() {
        let aabb = Aabb::new(-M, -2 * M, M, 2 * M);

        assert!(aabb.contains_point(-M, -2 * M));
        assert!(!aabb.contains_point(M + 1, 2 * M));
    }

    #[test]
    fn intersects_is_symmetric() {
        let a = Aabb::new(0, 0, 3 * M, 3 * M);
        let b = Aabb::new(2 * M, 2 * M, 5 * M, 5 * M);
        assert_eq!(a.intersects(&b), b.intersects(&a));
    }
}
