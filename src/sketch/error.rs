use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SketchError {
    // Plane errors
    #[error("Degenerate plane: x_dir and y_dir are collinear or zero-length")]
    DegeneratePlane,

    // Loop errors
    #[error("Loop is not closed: gap of {gap:.6} at curve index {index}")]
    OpenLoop { index: usize, gap: f64 },

    #[error("Loop has no curves")]
    EmptyLoop,

    // Curve errors
    #[error("Degenerate curve: zero or near-zero length")]
    DegenerateCurve,

    #[error("Invalid arc radius: must be positive, got {0}")]
    InvalidArcRadius(f64),

    #[error("Invalid arc: start and end points are not equidistant from center (r1={r1:.6}, r2={r2:.6})")]
    ArcRadiusMismatch { r1: f64, r2: f64 },

    #[error("Invalid arc: sweep angle is zero")]
    ZeroSweepAngle,

    #[error("Invalid circle: radius must be positive, got {0}")]
    InvalidCircleRadius(f64),

    #[error("Collinear points: cannot construct arc through three collinear points")]
    CollinearPoints,

    // Spline errors
    #[error("Unbounded spline parameter")]
    UnboundedSpline,

    #[error("Invalid B-spline: need at least {min} control points for degree {degree}, got {got}")]
    InsufficientControlPoints { min: usize, degree: usize, got: usize },

    // Builder errors
    #[error("Builder has no starting point: call move_to() first")]
    NoStartingPoint,

    #[error("Cannot close loop: need at least one curve")]
    CannotCloseEmpty,

    // Topology errors
    #[error("Failed to create truck edge: {0}")]
    TruckEdgeError(String),

    #[error("Failed to create truck wire: {0}")]
    TruckWireError(String),

    #[error("Failed to create truck face: {0}")]
    TruckFaceError(String),
}

pub type SketchResult<T> = Result<T, SketchError>;
