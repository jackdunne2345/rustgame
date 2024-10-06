pub mod raycaster {
    pub mod view {
        pub mod image;
        pub mod render;
    }
    pub mod model {
        pub mod vector;
    }
    pub mod controller {
        pub mod ray;
        pub mod sphere;
    }
}

use raycaster::model::vector::Vec3;
use raycaster::view::image::save_image;
use raycaster::view::render::render;
fn main() {
    let width = 800;
    let height = 600;
    let buffer: Vec<Vec<Vec3>> = render(width, height);
    save_image(width, height, buffer);
}
