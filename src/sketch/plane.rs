use crate::sketch::constants::*;
use crate::sketch::error::*;
use truck_geometry::prelude::*;

/// A plane in 3D space for lifting 2D sketches
#[derive(Clone, Debug)]
pub struct Plane {
    origin: Point3,
    x_dir: Vector3,
    y_dir: Vector3,
}

impl Plane {
    /// Create plane from origin and two direction vectors
    pub fn new(origin: Point3, x_dir: Vector3, y_dir: Vector3) -> SketchResult<Self> {
        // Validate non-collinear
        let normal = x_dir.cross(y_dir);
        if normal.magnitude() < DEGENERATE_TOLERANCE {
            return Err(SketchError::DegeneratePlane);
        }

        Ok(Self {
            origin,
            x_dir: x_dir.normalize(),
            y_dir: y_dir.normalize(),
        })
    }

    /// XY plane at origin
    pub fn xy() -> Self {
        Self {
            origin: Point3::origin(),
            x_dir: Vector3::unit_x(),
            y_dir: Vector3::unit_y(),
        }
    }

    /// XZ plane at origin
    #[allow(dead_code)]
    pub fn xz() -> Self {
        Self {
            origin: Point3::origin(),
            x_dir: Vector3::unit_x(),
            y_dir: Vector3::unit_z(),
        }
    }

    /// YZ plane at origin
    #[allow(dead_code)]
    pub fn yz() -> Self {
        Self {
            origin: Point3::origin(),
            x_dir: Vector3::unit_y(),
            y_dir: Vector3::unit_z(),
        }
    }

    /// Create plane at offset from XY
    #[allow(dead_code)]
    pub fn xy_at(z: f64) -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, z),
            x_dir: Vector3::unit_x(),
            y_dir: Vector3::unit_y(),
        }
    }

    /// Create from three points
    #[allow(dead_code)]
    pub fn from_three_points(p0: Point3, p1: Point3, p2: Point3) -> SketchResult<Self> {
        let x_dir = (p1 - p0).normalize();
        let temp = p2 - p0;
        let normal = x_dir.cross(temp).normalize();
        let y_dir = normal.cross(x_dir);

        Self::new(p0, x_dir, y_dir)
    }

    /// Normal vector
    pub fn normal(&self) -> Vector3 {
        self.x_dir.cross(self.y_dir).normalize()
    }

    /// Convert to truck Plane
    pub fn to_truck_plane(&self) -> SketchResult<truck_geometry::specifieds::Plane> {
        let normal = self.x_dir.cross(self.y_dir);
        if normal.magnitude() < DEGENERATE_TOLERANCE {
            return Err(SketchError::DegeneratePlane);
        }

        let p0 = self.origin;
        let p1 = self.origin + self.x_dir;
        let p2 = self.origin + self.y_dir;

        Ok(truck_geometry::specifieds::Plane::new(p0, p1, p2))
    }

    /// Lift 2D point to 3D
    pub fn lift_point(&self, p: Point2) -> Point3 {
        self.origin + self.x_dir * p.x + self.y_dir * p.y
    }

    /// Project 3D point to 2D (on this plane)
    #[allow(dead_code)]
    pub fn project_point(&self, p: Point3) -> Point2 {
        let v = p - self.origin;
        Point2::new(v.dot(self.x_dir), v.dot(self.y_dir))
    }

    // Getters
    #[allow(dead_code)]
    pub fn origin(&self) -> Point3 {
        self.origin
    }
    #[allow(dead_code)]
    pub fn x_dir(&self) -> Vector3 {
        self.x_dir
    }
    #[allow(dead_code)]
    pub fn y_dir(&self) -> Vector3 {
        self.y_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degenerate_plane() {
        let result = Plane::new(
            Point3::origin(),
            Vector3::unit_x(),
            Vector3::unit_x() * 2.0, // Collinear!
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_lift_point() {
        let plane = Plane::xy();
        let p2 = Point2::new(1.0, 2.0);
        let p3 = plane.lift_point(p2);
        assert!((p3 - Point3::new(1.0, 2.0, 0.0)).magnitude() < 1e-10);
    }
}
