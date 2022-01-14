//! # Graphical Models
//!
//! Section 2.1.4 recommends storing a collection of line pointers
//! alongside each point to allow efficient lookup of adjacent line
//! segments when deleting points. This is a structure of arrays
//! alternative that allows for more straight-forward that avoids
//! sticky ownership problems.

/// A collection of points and lines that define a particular model.
pub struct Model {
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    point_lines: Vec<Vec<usize>>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Line {
    p1: usize,
    p2: usize,
}

impl Model {
    pub fn new() -> Self {
        Model {
            points: Vec::new(),
            lines: Vec::new(),
            point_lines: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: u64, y: u64, z: u64, w: u64) {
        self.points.push(Point { x, y, z, w });
        self.point_lines.push(Vec::new());
    }

    pub fn add_line(&mut self, p1_index: usize, p2_index: usize) {
        let line = Line {
            p1: p1_index,
            p2: p2_index,
        };
        let i = self.lines.len();
        self.lines.push(line);

        self.point_lines[p1_index].push(i);
        self.point_lines[p2_index].push(i);
    }
}

#[cfg(test)]
mod test {
    use super::Line;
    use super::Model;
    use super::Point;

    #[test]
    fn add_points_and_line() {
        let mut model = Model::new();

        assert_eq!(model.points.len(), 0);
        assert_eq!(model.lines.len(), 0);
        assert_eq!(model.point_lines.len(), 0);

        model.add_point(0, 0, 0, 0);
        model.add_point(1, 1, 1, 0);

        assert_eq!(model.points.len(), 2);
        assert_eq!(
            model.points[0],
            Point {
                x: 0,
                y: 0,
                z: 0,
                w: 0
            }
        );
        assert_eq!(
            model.points[1],
            Point {
                x: 1,
                y: 1,
                z: 1,
                w: 0
            }
        );
        assert_eq!(model.point_lines.len(), 2);
        assert_eq!(model.point_lines[0].len(), 0);

        model.add_line(0, 1);

        assert_eq!(model.lines.len(), 1);
        assert_eq!(model.lines[0], Line { p1: 0, p2: 1 });
        assert_eq!(model.point_lines[0].len(), 1);
        assert_eq!(model.point_lines[1].len(), 1);
    }
}
