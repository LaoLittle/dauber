use crate::geom::point::Point;

#[derive(Clone)]
pub struct Path {
    verb: Vec<PathVerb>,
    points: Vec<Point>,
}

impl Path {
    #[inline]
    pub const fn new() -> Self {
        Self {
            verb: vec![],
            points: vec![],
        }
    }

    pub fn move_to(&mut self, at: Point) {
        self.verb.push(PathVerb::Move);
        self.points.push(at);
    }

    pub fn line_to(&mut self, to: Point) {
        self.verb.push(PathVerb::Line);
        self.points.push(to);
    }

    pub fn close(&mut self) {
        self.verb.push(PathVerb::Close);
    }

    pub fn iter(&self) -> Iter {
        Iter {
            verb: self.verb.iter(),
            points: self.points.iter(),
            last: Point::new(0., 0.),
        }
    }
}

#[derive(Copy, Clone)]
pub enum PathVerb {
    Move,  // 1 point
    Line,  // 2 points
    Quad,  // 3 points
    Cubic, // 4 points
    Close, // 0 points
}

#[derive(Copy, Clone)]
pub enum PathFillType {
    EvenOdd,
    Winding,
    InverseEvenOdd,
    InverseWinding,
}

#[derive(Clone)]
pub enum PathSegment {
    Move {
        to: Point,
    },
    Line {
        from: Point,
        to: Point,
    },
    Quadratic {
        from: Point,
        ctrl: Point,
        to: Point,
    },
    Cubic {
        from: Point,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
    },
    Close,
}

pub struct Iter<'a> {
    verb: std::slice::Iter<'a, PathVerb>,
    points: std::slice::Iter<'a, Point>,
    last: Point,
}

impl<'a> Iter<'a> {
    fn next_point(&mut self) -> Option<Point> {
        let pt = self.points.next().copied();

        if let Some(pt) = pt {
            self.last = pt;
        }

        pt
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = PathSegment;

    fn next(&mut self) -> Option<Self::Item> {
        match self.verb.next() {
            Some(PathVerb::Move) => Some(PathSegment::Move {
                to: self.next_point()?,
            }),
            Some(PathVerb::Line) => Some(PathSegment::Line {
                from: self.last,
                to: self.next_point()?,
            }),
            Some(PathVerb::Quad) => Some(PathSegment::Quadratic {
                from: self.last,
                ctrl: self.next_point()?,
                to: self.next_point()?,
            }),
            Some(PathVerb::Cubic) => Some(PathSegment::Cubic {
                from: self.last,
                ctrl1: self.next_point()?,
                ctrl2: self.next_point()?,
                to: self.next_point()?,
            }),
            Some(PathVerb::Close) => Some(PathSegment::Close),
            None => None,
        }
    }
}
