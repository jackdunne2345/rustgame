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
    let mut old_time = 0.0;
    let (mut rl, thread) = raylib::init().size(SCREEN_WIDTH, SCREEN_HEIGHT).title("RayCaster").build();
    while !rl.window_should_close() {
                let mut d = rl.begin_drawing(&thread);
                for x in 0..SCREEN_WIDTH {
                    let camera_x: f64 = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
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
                    let perp_wall_dist:f64;
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
                    let line_height: i32=(SCREEN_HEIGHT as f64/perp_wall_dist) as i32;
                    let mut draw_start=-line_height/2+SCREEN_HEIGHT/2;
                    if draw_start <0 {draw_start=0}
                    let mut draw_end=line_height/2+SCREEN_HEIGHT/2;
                    if draw_end>=SCREEN_HEIGHT {draw_end=SCREEN_HEIGHT-1}
                    let mut color_rgb: Color = match WORLD_MAP[map_x as usize][map_y as usize] {
                        1 => Color::RED,    
                        2 => Color::GREEN,   
                        3 => Color::BLUE,   
                        4 => Color::VIOLET,   
                        5 => Color::YELLOWGREEN,   
                        _ => Color::WHITE,   
                    };
                    if side ==1 {color_rgb=color_rgb.brightness(0.5)}
                    //draws the line using raylib
                    d.draw_line(x, draw_start, x,draw_end, color_rgb);
                    
                    old_time=time;
                    time=d.get_time();
                    let frame_time:f64=((time -old_time)/1000.0) as f64;
                    // print!("{}",1.0/frame_time);
                 
                    let move_speed=frame_time*10000.0;
                    let rot_speed=frame_time*3000.0;

                 
                    if d.is_key_down(KeyboardKey::KEY_W ){
                        if WORLD_MAP[(pos_x+dir_x*move_speed)as usize][pos_y as usize]==0 {pos_x+=dir_x*move_speed}
                        if WORLD_MAP[pos_x as usize][(pos_y+dir_y*move_speed)as usize]==0 {pos_y+=dir_y*move_speed}
                    }
                    if d.is_key_down(KeyboardKey::KEY_S ){
                        if WORLD_MAP[(pos_x-dir_x*move_speed)as usize][pos_y as usize]==0 {pos_x-=dir_x*move_speed}
                        if WORLD_MAP[pos_x as usize][(pos_y-dir_y*move_speed)as usize]==0 {pos_y-=dir_y*move_speed}
                    }
                    if d.is_key_down(KeyboardKey::KEY_A){
                        let old_dir_x=dir_x;
                        dir_x=dir_x* rot_speed.cos()-dir_y*rot_speed.sin();
                        dir_y=old_dir_x*rot_speed.sin()+dir_y* rot_speed.cos();
                        let old_plane_x=plane_x;
                        plane_x=plane_x* rot_speed.cos()-plane_y*rot_speed.sin();
                        plane_y=old_plane_x*rot_speed.sin()+plane_y* rot_speed.cos();
                    }
                    if d.is_key_down(KeyboardKey::KEY_D ){
                        let old_dir_x=dir_x;
                        dir_x=dir_x* (-rot_speed).cos()-dir_y*(-rot_speed).sin();
                        dir_y=old_dir_x*(-rot_speed).sin()+dir_y* (-rot_speed).cos();
                        let old_plane_x=plane_x;
                        plane_x=plane_x* (-rot_speed).cos()-plane_y*(-rot_speed).sin();
                        plane_y=old_plane_x*(-rot_speed).sin()+plane_y* (-rot_speed).cos();
                    }
                    
                 d.clear_background(Color::BLACK);
               
            }
}}
