use kurbo::{Point, Rect};

const MAX_CAPACITY: usize = 4;
const MAX_DEPTH: usize = 8;

/// An item stored in the quadtree with its bounding box.
#[derive(Debug, Clone)]
pub struct QtItem {
    pub bounds: Rect,
    pub id: String,
}

/// A node in the quadtree.
#[derive(Debug)]
pub struct Quadtree {
    boundary: Rect,
    depth: usize,
    items: Vec<QtItem>,
    children: Option<Box<[Quadtree; 4]>>,
}

impl Quadtree {
    pub fn new(boundary: Rect) -> Self {
        Self::with_depth(boundary, 0)
    }

    fn with_depth(boundary: Rect, depth: usize) -> Self {
        Quadtree {
            boundary,
            depth,
            items: Vec::new(),
            children: None,
        }
    }

    /// Insert an item into the quadtree.
    pub fn insert(&mut self, item: QtItem) {
        if !rects_overlap(self.boundary, item.bounds) {
            return;
        }

        if self.children.is_some() {
            self.insert_into_children(item);
            return;
        }

        self.items.push(item);

        if self.items.len() > MAX_CAPACITY && self.depth < MAX_DEPTH {
            self.subdivide();
        }
    }

    fn subdivide(&mut self) {
        let cx = (self.boundary.x0 + self.boundary.x1) / 2.0;
        let cy = (self.boundary.y0 + self.boundary.y1) / 2.0;
        let d = self.depth + 1;

        self.children = Some(Box::new([
            Quadtree::with_depth(Rect::new(self.boundary.x0, self.boundary.y0, cx, cy), d),
            Quadtree::with_depth(Rect::new(cx, self.boundary.y0, self.boundary.x1, cy), d),
            Quadtree::with_depth(Rect::new(self.boundary.x0, cy, cx, self.boundary.y1), d),
            Quadtree::with_depth(Rect::new(cx, cy, self.boundary.x1, self.boundary.y1), d),
        ]));

        let items = std::mem::take(&mut self.items);
        for item in items {
            self.insert_into_children(item);
        }
    }

    fn insert_into_children(&mut self, item: QtItem) {
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if rects_overlap(child.boundary, item.bounds) {
                    child.insert(item.clone());
                    return;
                }
            }
        }
    }

    /// Query all items whose bounds overlap with the given rectangle.
    pub fn query(&self, range: Rect) -> Vec<&QtItem> {
        let mut results = Vec::new();
        self.query_inner(range, &mut results);
        results
    }

    fn query_inner<'a>(&'a self, range: Rect, results: &mut Vec<&'a QtItem>) {
        if !rects_overlap(self.boundary, range) {
            return;
        }
        for item in &self.items {
            if rects_overlap(item.bounds, range) {
                results.push(item);
            }
        }
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_inner(range, results);
            }
        }
    }

    /// Find the nearest item to a given point (linear scan within candidates).
    pub fn nearest(&self, pt: Point) -> Option<&QtItem> {
        let search = Rect::new(pt.x - 50.0, pt.y - 50.0, pt.x + 50.0, pt.y + 50.0);
        let candidates = self.query(search);
        candidates.into_iter().min_by(|a, b| {
            let da = center_dist(a.bounds, pt);
            let db = center_dist(b.bounds, pt);
            da.partial_cmp(&db).unwrap()
        })
    }
}

/// AABB overlap test — returns true if two Rects share any area.
fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x0 < b.x1 && a.x1 > b.x0 && a.y0 < b.y1 && a.y1 > b.y0
}

fn center_dist(r: Rect, p: Point) -> f64 {
    let cx = (r.x0 + r.x1) / 2.0;
    let cy = (r.y0 + r.y1) / 2.0;
    ((cx - p.x).powi(2) + (cy - p.y).powi(2)).sqrt()
}
