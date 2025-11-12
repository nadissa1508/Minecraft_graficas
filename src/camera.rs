use crate::utils::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub aspect: f32,

    // Orbital camera parameters
    distance: f32,
    horizontal_angle: f32,
    vertical_angle: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, fov: f32, aspect: f32) -> Self {
        let distance = (position - target).length();
        let direction = (position - target).normalize();

        let horizontal_angle = direction.z.atan2(direction.x);
        let vertical_angle = direction.y.asin();

        Self {
            position,
            target,
            fov,
            aspect,
            distance,
            horizontal_angle,
            vertical_angle,
        }
    }

    // Get the forward direction vector (where camera is looking)
    fn get_forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    // Get the right direction vector (perpendicular to forward and up)
    fn get_right(&self) -> Vec3 {
        let forward = self.get_forward();
        forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize()
    }

    // Get the up direction vector
    fn get_up(&self) -> Vec3 {
        let forward = self.get_forward();
        let right = self.get_right();
        right.cross(&forward).normalize()
    }

    // === Rotation methods (arrow keys) ===
    pub fn rotate_around_target(&mut self, angle_delta: f32) {
        self.horizontal_angle += angle_delta.to_radians();
        self.update_position_and_target();
    }

    pub fn rotate_vertical(&mut self, angle_delta: f32) {
        self.vertical_angle += angle_delta.to_radians();
        self.vertical_angle = self.vertical_angle.clamp(-1.5, 1.5);
        self.update_position_and_target();
    }

    // === Zoom (UP/DOWN arrow or mouse wheel) ===
    pub fn zoom(&mut self, delta: f32) {
        self.distance -= delta;
        self.distance = self.distance.max(1.0).min(50.0);
        self.update_position_and_target();
    }

    // === WASD movement ===
    pub fn move_forward(&mut self, amount: f32) {
        let forward = self.get_forward();
        self.position = self.position + forward * amount;
        self.target = self.target + forward * amount;
    }

    pub fn move_backward(&mut self, amount: f32) {
        let forward = self.get_forward();
        self.position = self.position - forward * amount;
        self.target = self.target - forward * amount;
    }

    pub fn strafe_left(&mut self, amount: f32) {
        let right = self.get_right();
        self.position = self.position - right * amount;
        self.target = self.target - right * amount;
    }

    pub fn strafe_right(&mut self, amount: f32) {
        let right = self.get_right();
        self.position = self.position + right * amount;
        self.target = self.target + right * amount;
    }

    // === Vertical movement (Q/E keys) ===
    pub fn move_up(&mut self, amount: f32) {
        let up = Vec3::new(0.0, amount, 0.0);
        self.position = self.position + up;
        self.target = self.target + up;
    }

    pub fn move_down(&mut self, amount: f32) {
        let down = Vec3::new(0.0, -amount, 0.0);
        self.position = self.position + down;
        self.target = self.target + down;
    }

    fn update_position_and_target(&mut self) {
        let x = self.distance * self.vertical_angle.cos() * self.horizontal_angle.cos();
        let y = self.distance * self.vertical_angle.sin();
        let z = self.distance * self.vertical_angle.cos() * self.horizontal_angle.sin();

        self.position = self.target + Vec3::new(x, y, z);
    }

    // Generate a ray for pixel coordinates (u, v) in [0, 1]
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(&forward).normalize();

        let fov_rad = self.fov.to_radians();
        let half_height = (fov_rad / 2.0).tan();
        let half_width = self.aspect * half_height;

        let direction = forward
            + right * (2.0 * u - 1.0) * half_width
            + up * (1.0 - 2.0 * v) * half_height;

        Ray::new(self.position, direction.normalize())
    }
}
