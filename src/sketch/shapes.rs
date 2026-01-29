use crate::sketch::builder::SketchBuilder;
use crate::sketch::error::*;
use crate::sketch::loop2d::Loop2D;
use crate::sketch::primitives::{Circle2D, Curve2D};
use std::f64::consts::PI;
use truck_geometry::prelude::*;

/// Create common shapes easily
pub struct Shapes;

impl Shapes {
    /// Rectangle from corner and dimensions
    pub fn rectangle(corner: Point2, width: f64, height: f64) -> SketchResult<Loop2D> {
        SketchBuilder::new()
            .move_to(corner)
            .horizontal(width)?
            .vertical(height)?
            .horizontal(-width)?
            .close()
    }

    /// Rectangle centered at point
    #[allow(dead_code)]
    pub fn rectangle_centered(center: Point2, width: f64, height: f64) -> SketchResult<Loop2D> {
        let corner = Point2::new(center.x - width / 2.0, center.y - height / 2.0);
        Self::rectangle(corner, width, height)
    }

    /// Rectangle with rounded corners
    #[allow(dead_code)]
    pub fn rounded_rectangle(
        corner: Point2,
        width: f64,
        height: f64,
        radius: f64,
    ) -> SketchResult<Loop2D> {
        let r = radius.min(width / 2.0).min(height / 2.0);

        let p0 = Point2::new(corner.x + r, corner.y);
        let p1 = Point2::new(corner.x + width - r, corner.y);
        let p2 = Point2::new(corner.x + width, corner.y + r);
        let p3 = Point2::new(corner.x + width, corner.y + height - r);
        let p4 = Point2::new(corner.x + width - r, corner.y + height);
        let p5 = Point2::new(corner.x + r, corner.y + height);
        let p6 = Point2::new(corner.x, corner.y + height - r);
        let p7 = Point2::new(corner.x, corner.y + r);

        let c0 = Point2::new(corner.x + width - r, corner.y + r);
        let c1 = Point2::new(corner.x + width - r, corner.y + height - r);
        let c2 = Point2::new(corner.x + r, corner.y + height - r);
        let c3 = Point2::new(corner.x + r, corner.y + r);

        SketchBuilder::new()
            .move_to(p0)
            .line_to(p1)?
            .arc_to(p2, c0, true)?
            .line_to(p3)?
            .arc_to(p4, c1, true)?
            .line_to(p5)?
            .arc_to(p6, c2, true)?
            .line_to(p7)?
            .close_with_arc(c3, true)
    }

    /// Circle from center and radius
    pub fn circle(center: Point2, radius: f64) -> SketchResult<Loop2D> {
        let circle = Circle2D::new(center, radius)?;
        Loop2D::from_closed_curve(Curve2D::Circle(circle))
    }

    /// Regular polygon with n sides
    #[allow(dead_code)]
    pub fn regular_polygon(center: Point2, radius: f64, n: usize) -> SketchResult<Loop2D> {
        if n < 3 {
            return Err(SketchError::DegenerateCurve);
        }

        let angle_step = 2.0 * PI / n as f64;
        let mut builder = SketchBuilder::new();

        // Start at top
        let start = Point2::new(center.x, center.y + radius);
        builder = builder.move_to(start);

        for i in 1..n {
            let angle = PI / 2.0 + i as f64 * angle_step;
            let pt = Point2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin());
            builder = builder.line_to(pt)?;
        }

        builder.close()
    }

    /// Slot shape (rectangle with semicircle ends)
    #[allow(dead_code)]
    pub fn slot(center: Point2, length: f64, width: f64, horizontal: bool) -> SketchResult<Loop2D> {
        let r = width / 2.0;
        let half_length = length / 2.0 - r;

        if horizontal {
            let p0 = Point2::new(center.x - half_length, center.y - r);
            let p1 = Point2::new(center.x + half_length, center.y - r);
            let p2 = Point2::new(center.x + half_length, center.y + r);
            let p3 = Point2::new(center.x - half_length, center.y + r);

            let c_right = Point2::new(center.x + half_length, center.y);
            let c_left = Point2::new(center.x - half_length, center.y);

            SketchBuilder::new()
                .move_to(p0)
                .line_to(p1)?
                .arc_to(p2, c_right, true)?
                .line_to(p3)?
                .close_with_arc(c_left, true)
        } else {
            let p0 = Point2::new(center.x - r, center.y - half_length);
            let p1 = Point2::new(center.x - r, center.y + half_length);
            let p2 = Point2::new(center.x + r, center.y + half_length);
            let p3 = Point2::new(center.x + r, center.y - half_length);

            let c_top = Point2::new(center.x, center.y + half_length);
            let c_bottom = Point2::new(center.x, center.y - half_length);

            SketchBuilder::new()
                .move_to(p0)
                .line_to(p1)?
                .arc_to(p2, c_top, true)?
                .line_to(p3)?
                .close_with_arc(c_bottom, true)
        }
    }

    /// L-shape profile
    #[allow(dead_code)]
    pub fn l_shape(
        corner: Point2,
        width: f64,
        height: f64,
        thickness: f64,
    ) -> SketchResult<Loop2D> {
        SketchBuilder::new()
            .move_to(corner)
            .horizontal(width)?
            .vertical(thickness)?
            .horizontal(-(width - thickness))?
            .vertical(height - thickness)?
            .horizontal(-thickness)?
            .close()
    }

    /// T-shape profile
    #[allow(dead_code)]
    pub fn t_shape(
        base_center: Point2,
        flange_width: f64,
        flange_thickness: f64,
        web_height: f64,
        web_thickness: f64,
    ) -> SketchResult<Loop2D> {
        let half_flange = flange_width / 2.0;
        let half_web = web_thickness / 2.0;

        let start = Point2::new(base_center.x - half_flange, base_center.y);

        SketchBuilder::new()
            .move_to(start)
            .horizontal(flange_width)?
            .vertical(flange_thickness)?
            .horizontal(-(half_flange - half_web))?
            .vertical(web_height - flange_thickness)?
            .horizontal(-web_thickness)?
            .vertical(-(web_height - flange_thickness))?
            .horizontal(-(half_flange - half_web))?
            .close()
    }

    /// Hexagon (flat top orientation)
    #[allow(dead_code)]
    pub fn hexagon(center: Point2, size: f64) -> SketchResult<Loop2D> {
        Self::regular_polygon(center, size, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Shapes::rectangle(Point2::origin(), 10.0, 5.0).unwrap();
        assert!(rect.validate(1e-9).is_ok());
    }

    #[test]
    fn test_circle() {
        let circle = Shapes::circle(Point2::origin(), 10.0).unwrap();
        assert!(circle.validate(1e-9).is_ok());
    }

    #[test]
    fn test_regular_polygon() {
        let hex = Shapes::regular_polygon(Point2::origin(), 10.0, 6).unwrap();
        assert!(hex.validate(1e-9).is_ok());
    }
}
