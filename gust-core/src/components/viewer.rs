use gust_math::vectors::vect3::Vect3;

pub trait Viewer {
    fn position(&self) -> Vect3;
    fn direction(&self) -> Vect3;
    fn up(&self) -> Vect3;
    fn fov(&self) -> f32;
    fn z_far(&self) -> f32;
    fn z_near(&self) -> f32;
    fn aspect_ratio(&self) -> f32;
    fn get_perspective(&self) -> [[f32; 4]; 4] {
        let f = 1.0 / (self.fov() / 2.0).tan();
        [
            [f * self.aspect_ratio(), 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (self.z_far() + self.z_near()) / (self.z_far() - self.z_near()), 1.0],
            [0.0, 0.0, -(2.0 * self.z_far() * self.z_near()) / (self.z_far() - self.z_near()), 0.0],
        ]
    }
   
    fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = self.direction().clone().normalize();
        let s = self.up().cross(&f).normalize();
        let u = f.cross(&s).normalize();

        let p = [
            -self.position().dot(&s),
            -self.position().dot(&u),
            -self.position().dot(&f),
        ];

        [
            [s.x, u.x, f.x, 0.0],
            [s.y, u.y, f.y, 0.0],
            [s.z, u.z, f.z, 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}