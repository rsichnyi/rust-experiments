//#![windows_subsystem = "windows"]

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

    pub fn draw(&self) {
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }
}

pub struct Player {
    pub texture: Texture2D,
    pub rect: Rect,
    pub speed: Vec2
}

impl Player {
    pub fn new(x: u16, y: u16) -> Player {
        let width = TILE_SIZE;
        let height = TILE_SIZE;

        let image = Image::gen_image_color(width - 4, height - 4, RED);
        Player {
            texture: Texture2D::from_image(&image),
            rect: Rect::new(x.into(), y.into(), width.into(), height.into()),
            ..Default::default()
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.rect.x + 2., self.rect.y + 2., WHITE);
    }

    pub fn update(&mut self, boundaries: Rect) {
        self.rect.move_to(self.rect.point() + self.speed);

        if self.rect.left() < boundaries.left() {
            self.rect.x = boundaries.left()
        }

        if self.rect.top() < boundaries.top() {
            self.rect.y = boundaries.top()
        }

        if self.rect.right() > boundaries.right() {
            self.rect.x = boundaries.right() - self.rect.w
        }

        if self.rect.bottom() > boundaries.bottom() {
            self.rect.y = boundaries.bottom() - self.rect.h
        }

        self.speed = vec2(0., 0.);
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            texture: Texture2D::empty(),
            rect: Rect::new(0., 0., 0., 0.),
            speed: vec2(0., 0.)
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

    pub fn update(&mut self, boundaries: Rect) {
        self.camera.target += self.speed;
        let camera_rect = (
            self.camera.screen_to_world(vec2(0., 0.)), 
            self.camera.screen_to_world(vec2(screen_width(), screen_height()))
        );

        if camera_rect.0.x < boundaries.left() {
            self.camera.target.x = boundaries.left() + self.width / 2.
        }
        
        if camera_rect.0.y < boundaries.top() {
            self.camera.target.y = boundaries.top() + self.height / 2.
        }

        if camera_rect.1.x > boundaries.right() {
            self.camera.target.x = boundaries.right() - self.width / 2.
        }

        if camera_rect.1.y > boundaries.bottom() {
            self.camera.target.y = boundaries.bottom() - self.height / 2.
        }
    }
}

pub struct World {
    pub tiles: Vec<Tile>,
    player: Player,
    player_camera: PlayerCamera,
    boundaries: Rect
}

impl World {
    pub fn new(map_data: Vec<Vec<i32>>) -> World {
        let mut tiles = vec![];
        let mut player = Player::new(0, 0);

        for (row_idx, l) in map_data.iter().enumerate() {
            for (col_idx, x) in l.iter().enumerate() {
                if *x == 1 {
                    tiles.push(Tile::new(col_idx as u16 * TILE_SIZE, row_idx as u16 * TILE_SIZE));
                }
                if *x == 2 {
                    player.rect.x = (col_idx as u16 * TILE_SIZE).into();
                    player.rect.y = (row_idx as u16 * TILE_SIZE).into();
                }
            }
        }

        World {
            tiles,
            boundaries: Rect::new(
                0.,
                0.,
                (map_data[0].len() as u16 * TILE_SIZE) as f32,
                (map_data.len() as u16 * TILE_SIZE) as f32
            ),
            player,
            player_camera: PlayerCamera::new(1280., 720.)
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter() {
            tile.draw();
        }
        self.update_player();
        self.update_camera();
    }

    pub fn update_player(&mut self) {
        let player_speed = 4.;

        if is_key_down(KeyCode::Right) {
            self.player.speed.x = player_speed
        }
        if is_key_down(KeyCode::Left) {
            self.player.speed.x = -player_speed
        }
        if is_key_down(KeyCode::Down) {
            self.player.speed.y = player_speed
        }
        if is_key_down(KeyCode::Up) {
            self.player.speed.y = -player_speed
        }
        self.player.update(self.boundaries);

        self.player.draw();
    }

    pub fn update_camera(&mut self) {

        let target = self.player.rect.point() + self.player.rect.size() / 2.;
        // println!("target: {}, {}, camera: {}, {}", target.x, target.y, self.player_camera.camera.target.x, self.player_camera.camera.target.y);

        let offset = vec2(
            target.x - self.player_camera.camera.target.x,
            target.y - self.player_camera.camera.target.y
        );

        if offset.length() < 1. / 16. {
            self.player_camera.camera.target = target
        } else {
            self.player_camera.speed = offset / 32.;
        }

        self.player_camera.update(self.boundaries);

        set_camera(&self.player_camera.camera);
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let level_map = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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

        draw_text_ex("Use arrows to move player", 100.0, 40.0, TextParams::default());

        world.update();

        next_frame().await
    }
}