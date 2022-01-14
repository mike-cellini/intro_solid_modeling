//! # Graphical Models
//!
//! Section 2.1.4 recommends storing a collection of line pointers
//! alongside each point to allow efficient lookup of adjacent line
//! segments when deleting points. This is an alternative implementation
//! utilizing BTreeMaps to store each Point/Line alongside a handle that
//! can be used to reference it. This avoids sticky borrow checker problems
//! while also providing a reasonable method for storing/accessing points
//! in order of creation.

use std::collections::BTreeMap;

/// A collection of points and lines that define a particular model.
pub struct Model {
    pub points: BTreeMap<PointHandle, Point>,
    pub lines: BTreeMap<LineHandle, Line>,
}

/// The homogenous coordinate representation of a point. See section 2.1.1.
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
    lines: Vec<LineHandle>,
}

/// Represents a handle used to reference a point.
type PointHandle = usize;

/// A line defined by a reference to the location of the point in the Model structure.
#[derive(Eq, PartialEq, Debug)]
pub struct Line {
    p1: PointHandle,
    p2: PointHandle,
}

/// Represents a handle used to reference a line.
type LineHandle = usize;

impl Default for Model {
    fn default() -> Model {
        Model::new()
    }
}

impl Model {
    /// Creates a new, empty Model
    pub fn new() -> Self {
        Model {
            points: BTreeMap::new(),
            lines: BTreeMap::new(),
        }
    }

    /// Add a point to be stored with the model
    pub fn add_point(&mut self, x: i64, y: i64, z: i64, w: i64) -> PointHandle {
        let handle = match self.points.last_key_value() {
            Some((handle, _)) => handle + 1,
            None => 1
        };

        self.points.insert(handle, Point {
            x,
            y,
            z,
            w,
            lines: Vec::new(),
        });

        handle
    }

    fn add_line_to_point(&mut self, point_handle: PointHandle, line_handle: LineHandle) {
        if let Some(p) = self.points.get_mut(&point_handle) {
            p.lines.push(line_handle);
        }
    }

    /// Add a line to be stored with the model
    pub fn add_line(&mut self, p1: PointHandle, p2: PointHandle) -> LineHandle {
        let handle = match self.lines.last_key_value() {
            Some((handle, _)) => handle + 1,
            None => 1};

        let line = Line { p1, p2 };
        self.lines.insert(handle, line);

        self.add_line_to_point(p1, handle);
        self.add_line_to_point(p2, handle);

        handle
    }

    /// Gets all the handles for all lines associated with a point
    pub fn get_point_lines(&self, handle: PointHandle) -> Vec<LineHandle> {
        self.points[&handle].lines.to_vec()
    }

    /// Deletes a line from the model
    pub fn del_line(&mut self, handle: LineHandle) {
        self.lines.remove(&handle);
    }

    /// Deletes a point from the model, also cleans up any associated lines.
    pub fn del_point(&mut self, handle: PointHandle) {
        for &handle in self.get_point_lines(handle).iter() {
            self.lines.remove(&handle);
        }
        self.points.remove(&handle);
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

        let p1 = model.add_point(0, 0, 0, 0);
        let p2 = model.add_point(1, 1, 1, 0);

        assert_eq!(model.points.len(), 2);
        assert_eq!(
            model.points[&p1],
            Point {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
                lines: Vec::new(),
            }
        );
        assert_eq!(
            model.points[&p2],
            Point {
                x: 1,
                y: 1,
                z: 1,
                w: 0,
                lines: Vec::new(),
            }
        );
        assert_eq!(model.lines.len(), 0);

        let l1 = model.add_line(p1, p2);

        assert_eq!(model.lines.len(), 1);
        assert_eq!(model.lines[&l1], Line { p1, p2 });
        assert_eq!(model.points[&p1].lines.len(), 1);
        assert_eq!(model.points[&p2].lines.len(), 1);
        assert_eq!(model.points[&p1].lines[0], l1);
        assert_eq!(model.points[&p2].lines[0], l1);
    }

    #[test]
    fn del_points_and_line() {
        use super::Model;

        let mut model = Model::new();

        let p1 = model.add_point(0, 0, 0, 0);
        let p2 = model.add_point(1, 1, 1, 1);
        let p3 = model.add_point(2, 2, 2, 2);

        let _l1 = model.add_line(p1, p2);
        let l2 = model.add_line(p2, p3);
        
        model.del_point(p1);

        assert_eq!(model.lines.len(), 1);
        assert_eq!(model.points.len(), 2);

        model.del_line(l2);
        assert_eq!(model.lines.len(), 0);
        assert_eq!(model.points.len(), 2);
    }
}
