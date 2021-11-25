#![windows_subsystem = "windows"]

use macroquad::prelude::*;

const TILE_SIZE: u16 = 64;

fn window_conf() -> Conf {
    Conf {
        window_title: "whatever".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

pub struct Tile {
    pub texture: Texture2D,
    pub rect: Rect
}

impl Tile {
    pub fn new(x: u16, y: u16) -> Tile {
        let width = TILE_SIZE;
        let height = TILE_SIZE;

        let image = Image::gen_image_color(width, height, LIGHTGRAY);
        Tile {
            texture: Texture2D::from_image(&image),
            rect: Rect::new(x.into(), y.into(), width.into(), height.into())
        }
    }
}

pub struct PlayerCamera {
    camera: Camera2D,
    width: f32,
    height: f32,
    speed: Vec2
}

impl PlayerCamera {
    pub fn new(width: f32, height: f32) -> PlayerCamera {
        PlayerCamera {
            camera: Camera2D {
                target: vec2(width / 2., height / 2.),
                zoom: vec2(1. / width * 2., -1. / height * 2.),
                ..Default::default()
            },
            width,
            height,
            speed: vec2(0., 0.)
        }
    }

    pub fn update(&mut self, max_x: f32, max_y: f32) {
        self.camera.target += self.speed;
        let camera_pos = (
            self.camera.screen_to_world(vec2(0., 0.)), 
            self.camera.screen_to_world(vec2(screen_width(), screen_height()))
        );

        if camera_pos.0.x < 0. {
            self.camera.target.x = self.width / 2.;
            self.speed.x = -self.speed.x
        }
        
        if camera_pos.0.y < 0. {
            self.camera.target.y = self.height / 2.;
            self.speed.y = -self.speed.y
        }

        if camera_pos.1.x > max_x {
            self.camera.target.x = max_x - self.width / 2.;
            self.speed.x = -self.speed.x
        }
        
        if camera_pos.1.y > max_y {
            self.camera.target.y = max_y - self.height / 2.;
            self.speed.y = -self.speed.y
        }
    }
}

pub struct World {
    pub tiles: Vec<Tile>,
    player_camera: PlayerCamera,
    width: f32,
    height: f32
}

impl World {
    pub fn new(map_data: Vec<Vec<i32>>) -> World {
        let mut tiles = vec![];

        for (row_idx, l) in map_data.iter().enumerate() {
            for (col_idx, x) in l.iter().enumerate() {
                if *x == 1 {
                    tiles.push(Tile::new(col_idx as u16 * TILE_SIZE, row_idx as u16 * TILE_SIZE));
                }
            }
        }

        World {
            tiles,
            width: (map_data[0].len() as u16 * TILE_SIZE) as f32,
            height: (map_data.len() as u16 * TILE_SIZE) as f32,
            player_camera: PlayerCamera::new(1280., 720.)
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter() {
            draw_tile(tile);
        }
        self.update_camera()
    }

    pub fn update_camera(&mut self) {
        
        let mut camera_speed = vec2(0., 0.);

        if is_key_down(KeyCode::Right) {
            camera_speed.x = 4.
        }
        if is_key_down(KeyCode::Left) {
            camera_speed.x = -4.
        }
        if is_key_down(KeyCode::Down) {
            camera_speed.y = 4.
        }
        if is_key_down(KeyCode::Up) {
            camera_speed.y = -4.
        }

        self.player_camera.speed = camera_speed;

        self.player_camera.update(self.width, self.height);

        set_camera(&self.player_camera.camera);
    }
}

fn draw_tile(t: &Tile) {
    draw_texture(t.texture, t.rect.x, t.rect.y, WHITE);
}

#[macroquad::main(window_conf)]
async fn main() {

    let level_map = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]
    ];

    let mut world = World::new(level_map);

    loop {

        clear_background(BLACK);

        draw_text_ex("Use arrows to move camera", 100.0, 40.0, TextParams::default());

        world.update();

        next_frame().await
    }
}