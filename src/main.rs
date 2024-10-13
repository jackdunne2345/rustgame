// pub mod raycaster {
//     pub mod view {
//         pub mod image;
//         pub mod render;
//     }
//     pub mod model {
//         pub mod vector;
//     }
//     pub mod controller {
//         pub mod ray;
//         pub mod sphere;
//     }
// }

// use raycaster::model::vector::Vec3;
// use raycaster::view::image::save_image;
// use raycaster::view::render::render;
// fn main() {
//     let width = 800;
//     let height = 600;
//     let buffer: Vec<Vec<Vec3>> = render(width, height);
//     save_image(width, height, buffer);
// }
use raylib::prelude::*;

const MAP_WIDTH: usize  = 24;
const MAP_HEIGHT: usize  = 24;
const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32  = 480;

const WORLD_MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

fn main() {
    let mut pos_x = 22.0;
    let mut pos_y = 12.0;
    let mut dir_x=-1.0;
    let mut dir_y=0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;
    let mut time = 0.0;
    let mut olde_time = 0.0;
    let (mut rl, thread) = raylib::init().size(SCREEN_WIDTH, SCREEN_HEIGHT).title("RayCaster").build();
    while !rl.window_should_close() {
                let mut d = rl.begin_drawing(&thread);
                for i in 0..SCREEN_WIDTH {
                    let camera_x: f64 = 2.0 * i as f64 / SCREEN_WIDTH as f64 - 1.0;
                    let ray_dir_x:f64=dir_x +plane_x*camera_x;
                    let ray_dir_y:f64=dir_y +plane_y*camera_x;

                    //which box we are in
                    let mut map_x:i32=pos_x as i32;
                    let mut map_y:i32=pos_y as i32;

                    //length of ray from current position to next x or y-side
                    let mut side_dist_x:f64;
                    let mut side_dist_y:f64;

                    //length of ray from one x or y side to next x or y side
                    let delta_dist_x :f64= if ray_dir_x == 0.0 { 1e30} else { (1.0/ray_dir_x).abs() };
                    let delta_dist_y :f64= if ray_dir_y == 0.0 { 1e30} else { (1.0/ray_dir_y).abs() };
                    let mut perp_wall_dist:f64;
                    // Direction to step in x or y-direction (either +1 or -1)
                    let step_x: i32; // Declare variable for X direction
                    let step_y: i32; // Declare variable for Y direction

                    let mut hit: i32 = 0; // Was there a wall hit?
                    let mut side: i32=2; // Was a NS or EW wall hit?

                    if ray_dir_x < 0.0 {
                        step_x=-1;
                        side_dist_x= (pos_x - map_x as f64)*delta_dist_x;
                    }else{
                        step_x=1;
                        side_dist_x=(map_x as f64 +1.0 -pos_x)*delta_dist_x;
                    }

                    if ray_dir_y <0.0{
                        step_y=-1;
                        side_dist_y =(pos_y - map_y as f64)*delta_dist_y;
                    }else{
                        step_y=-1;
                        side_dist_y=(map_y as f64+1.0 - pos_y)*delta_dist_y;
                    }
                    //perform DDA
                    while hit ==0{
                        if side_dist_x < side_dist_y{
                            side_dist_x +=delta_dist_x;
                            map_x+=step_x;
                            side=0;
                        }else{
                            side_dist_y+=delta_dist_y;
                            map_y+=step_y;
                            side=1;
                        }
                        if WORLD_MAP[map_x as usize][map_y as usize]>0{hit=1;}
                    }
                    if side == 0{ perp_wall_dist=side_dist_x -delta_dist_x}
                        else{ perp_wall_dist=side_dist_y-delta_dist_y}
                 
                    
                }
               
            }
}
