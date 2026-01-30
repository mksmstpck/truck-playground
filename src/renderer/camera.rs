use glam::{Mat4, Vec3};

pub struct OrbitCamera {
    /// Point the camera orbits around
    pub target: Vec3,

    /// Distance from target
    pub distance: f32,

    /// Horizontal angle (radians, around Y axis)
    pub azimuth_rad: f32,

    /// Vertical angle (radians, from horizontal)
    pub elevation_rad: f32,

    /// Field of view (radians)
    pub fov_rad: f32,

    /// Near clipping plane
    pub near: f32,

    /// Far clipping plane
    pub far: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 100.0,
            azimuth_rad: std::f32::consts::FRAC_PI_4, // 45°
            elevation_rad: std::f32::consts::FRAC_PI_6, // 30°
            fov_rad: std::f32::consts::FRAC_PI_4,     // 45°
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl OrbitCamera {
    /// Calculate camera position from spherical coordinates
    pub fn eye_position(&self) -> Vec3 {
        let x = self.distance * self.elevation_rad.cos() * self.azimuth_rad.sin();
        let y = self.distance * self.elevation_rad.sin();
        let z = self.distance * self.elevation_rad.cos() * self.azimuth_rad.cos();

        self.target + Vec3::new(x, y, z)
    }

    /// View matrix (world → camera space)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.eye_position(),
            self.target,
            Vec3::Y, // Up vector
        )
    }

    /// Projection matrix (camera → clip space)
    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh(self.fov_rad, aspect_ratio, self.near, self.far)
    }

    /// Combined view-projection matrix
    pub fn view_projection(&self, aspect_ratio: f32) -> Mat4 {
        self.projection_matrix(aspect_ratio) * self.view_matrix()
    }

    /// Rotate camera (from mouse drag)
    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        self.azimuth_rad -= delta_x * 0.01;
        self.elevation_rad += delta_y * 0.01;

        // Clamp elevation to avoid flipping
        self.elevation_rad = self.elevation_rad.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );
    }

    /// Zoom (from scroll wheel)
    pub fn zoom(&mut self, delta: f32) {
        self.distance *= 1.0 - delta * 0.1;
        self.distance = self.distance.clamp(1.0, 1000.0);
    }
}
