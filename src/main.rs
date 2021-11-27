//#![windows_subsystem = "windows"]

use std::collections::HashMap;
use macroquad::prelude::*;

const TILE_SIZE: u16 = 32;
const TILE_SIZE_FLOAT: f32 = TILE_SIZE as f32;

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
    pub fn new(x: f32, y: f32, color: Color) -> Tile {
        let image = Image::gen_image_color(TILE_SIZE, TILE_SIZE, color);
        Tile {
            texture: Texture2D::from_image(&image),
            rect: Rect::new(x, y, TILE_SIZE_FLOAT, TILE_SIZE_FLOAT)
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.rect.x, self.rect.y, WHITE);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PlayerAnimation {
    Idle,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight
}

pub struct Player {
    texture: Texture2D,
    pub rect: Rect,
    pub speed: Vec2,
    animations: HashMap<PlayerAnimation, Vec<Texture2D>>,
    animation: PlayerAnimation,
    animation_frame: f32,
}

impl Player {
    pub async fn new(x: f32, y: f32) -> Player {

        let mut animations = HashMap::new();
        animations.insert(PlayerAnimation::Idle, vec![
            load_texture("assets/knight/move_down/0.png").await.unwrap()
        ]);
        animations.insert(PlayerAnimation::MoveUp, vec![
            load_texture("assets/knight/move_up/0.png").await.unwrap(),
            load_texture("assets/knight/move_up/1.png").await.unwrap(),
            load_texture("assets/knight/move_up/2.png").await.unwrap(),
            load_texture("assets/knight/move_up/3.png").await.unwrap(),
            load_texture("assets/knight/move_up/4.png").await.unwrap(),
            load_texture("assets/knight/move_up/5.png").await.unwrap(),
            load_texture("assets/knight/move_up/6.png").await.unwrap(),
            load_texture("assets/knight/move_up/7.png").await.unwrap(),
            load_texture("assets/knight/move_up/8.png").await.unwrap()
        ]);
        animations.insert(PlayerAnimation::MoveDown, vec![
            load_texture("assets/knight/move_down/0.png").await.unwrap(),
            load_texture("assets/knight/move_down/1.png").await.unwrap(),
            load_texture("assets/knight/move_down/2.png").await.unwrap(),
            load_texture("assets/knight/move_down/3.png").await.unwrap(),
            load_texture("assets/knight/move_down/4.png").await.unwrap(),
            load_texture("assets/knight/move_down/5.png").await.unwrap(),
            load_texture("assets/knight/move_down/6.png").await.unwrap(),
            load_texture("assets/knight/move_down/7.png").await.unwrap(),
            load_texture("assets/knight/move_down/8.png").await.unwrap()
        ]);
        animations.insert(PlayerAnimation::MoveLeft, vec![
            load_texture("assets/knight/move_left/0.png").await.unwrap(),
            load_texture("assets/knight/move_left/1.png").await.unwrap(),
            load_texture("assets/knight/move_left/2.png").await.unwrap(),
            load_texture("assets/knight/move_left/3.png").await.unwrap(),
            load_texture("assets/knight/move_left/4.png").await.unwrap(),
            load_texture("assets/knight/move_left/5.png").await.unwrap(),
            load_texture("assets/knight/move_left/6.png").await.unwrap(),
            load_texture("assets/knight/move_left/7.png").await.unwrap(),
            load_texture("assets/knight/move_left/8.png").await.unwrap()
        ]);
        animations.insert(PlayerAnimation::MoveRight, vec![
            load_texture("assets/knight/move_right/0.png").await.unwrap(),
            load_texture("assets/knight/move_right/1.png").await.unwrap(),
            load_texture("assets/knight/move_right/2.png").await.unwrap(),
            load_texture("assets/knight/move_right/3.png").await.unwrap(),
            load_texture("assets/knight/move_right/4.png").await.unwrap(),
            load_texture("assets/knight/move_right/5.png").await.unwrap(),
            load_texture("assets/knight/move_right/6.png").await.unwrap(),
            load_texture("assets/knight/move_right/7.png").await.unwrap(),
            load_texture("assets/knight/move_right/8.png").await.unwrap()
        ]);

        Player {
            texture: animations.get(&PlayerAnimation::Idle).unwrap()[0],
            animations,
            rect: Rect::new(x.into(), y.into(), TILE_SIZE_FLOAT, TILE_SIZE_FLOAT),
            ..Default::default()
        }
    }

    pub fn animate(&mut self) {

        let mut animation = PlayerAnimation::Idle;

        if self.speed.x > 0. {
            animation = PlayerAnimation::MoveRight;
        } else if self.speed.x < 0. {
            animation = PlayerAnimation::MoveLeft;
        } else if self.speed.y > 0. {
            animation = PlayerAnimation::MoveDown;
        } else if self.speed.y < 0. {
            animation = PlayerAnimation::MoveUp;
        }

        if animation != self.animation {
            self.animation = animation;
            self.animation_frame = 0.;
        }

        let animation_frames = self.animations.get(&self.animation).unwrap();

        self.animation_frame += 0.08;
        if self.animation_frame >= animation_frames.len() as f32 {
            self.animation_frame = 0.;
        }
        self.texture = animation_frames[(self.animation_frame as usize)];
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.rect.x - 8., self.rect.y - 16., WHITE);
    }

    pub fn process_input(&mut self) {
        let player_speed = 1.;

        if is_key_down(KeyCode::Right) {
            self.speed.x = player_speed
        }
        if is_key_down(KeyCode::Left) {
            self.speed.x = -player_speed
        }
        if is_key_down(KeyCode::Down) {
            self.speed.y = player_speed
        }
        if is_key_down(KeyCode::Up) {
            self.speed.y = -player_speed
        }
    }

    pub fn update(&mut self){
        self.process_input();
        self.animate();
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            texture: Texture2D::empty(),
            rect: Rect::new(0., 0., 0., 0.),
            speed: vec2(0., 0.),
            animations: HashMap::new(),
            animation: PlayerAnimation::Idle,
            animation_frame: 0.,
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
    tiles: Vec<Tile>,
    player: Player,
    player_camera: PlayerCamera,
    boundaries: Rect
}

pub fn is_overlaped(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.left() < rect2.right()
        && rect1.right() > rect2.left()
        && rect1.top() < rect2.bottom()
        && rect1.bottom() > rect2.top()
}

impl World {
    pub async fn new(map_data: Vec<Vec<i32>>) -> World {
        let mut tiles = vec![];
        let mut player = Player::new(0., 0.).await;

        for (row_idx, l) in map_data.iter().enumerate() {
            for (col_idx, x) in l.iter().enumerate() {
                if *x == 1 {
                    tiles.push(Tile::new(col_idx as f32 * TILE_SIZE_FLOAT, row_idx as f32 * TILE_SIZE_FLOAT, LIGHTGRAY));
                }
                if *x == 2 {
                    player.rect.x = col_idx as f32 * TILE_SIZE_FLOAT;
                    player.rect.y = row_idx as f32 * TILE_SIZE_FLOAT;
                }
            }
        }

        World {
            tiles,
            boundaries: Rect::new(
                0.,
                0.,
                map_data[0].len() as f32 * TILE_SIZE_FLOAT,
                map_data.len() as f32 * TILE_SIZE_FLOAT
            ),
            player,
            player_camera: PlayerCamera::new(TILE_SIZE_FLOAT * 20., TILE_SIZE_FLOAT * 11.25)
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
        self.player.update();

        // splitting vertical/horizontal movement here
        // we need to know of player is to the right/left of the tile or to the bottom/top
        // can not determine that if we apply both movements at the same time (or can we?)

        // vertical movement
        if self.player.speed.y != 0. {
            self.player.rect.y += self.player.speed.y;

            if self.player.rect.top() < self.boundaries.top() {
                self.player.rect.y = self.boundaries.top()
            }

            if self.player.rect.bottom() > self.boundaries.bottom() {
                self.player.rect.y = self.boundaries.bottom() - self.player.rect.h
            }

            for tile in self.tiles.iter() {
                if is_overlaped(&self.player.rect, &tile.rect) {
                    if self.player.speed.y > 0. {
                        // we were moving down and actually moved inside the other tile, place player on top of the tile
                        self.player.rect.y = tile.rect.top() - self.player.rect.h
                    }
                    if self.player.speed.y < 0. {
                        // moving up, place player below the tile
                        self.player.rect.y = tile.rect.bottom()
                    }
                }
            }
        }

        // horizontal movement
        if self.player.speed.x != 0. {
            self.player.rect.x += self.player.speed.x;

            if self.player.rect.left() < self.boundaries.left() {
                self.player.rect.x = self.boundaries.left()
            }

            if self.player.rect.right() > self.boundaries.right() {
                self.player.rect.x = self.boundaries.right() - self.player.rect.w
            }

            for tile in self.tiles.iter() {
                if is_overlaped(&self.player.rect, &tile.rect) {
                    if self.player.speed.x > 0. {
                        // moving right, place the player on the left of tile
                        self.player.rect.x = tile.rect.left() - self.player.rect.w
                    }
                    if self.player.speed.x < 0. {
                        // moving left, place the player on the right of tile
                        self.player.rect.x = tile.rect.right()
                    }
                }
            }
        }

        // reset the speed since it's reapplied on the next frame
        self.player.speed = vec2(0., 0.);

        self.player.draw();
    }

    pub fn update_camera(&mut self) {

        let target = self.player.rect.point() + self.player.rect.size() / 2.;
        // println!("target: {}, {}, camera: {}, {}", target.x, target.y, self.player_camera.camera.target.x, self.player_camera.camera.target.y);

        let offset = vec2(
            target.x - self.player_camera.camera.target.x,
            target.y - self.player_camera.camera.target.y
        );

        if offset.length() < 1. / (TILE_SIZE_FLOAT / 4.) {
            self.player_camera.camera.target = target
        } else {
            self.player_camera.speed = offset / (TILE_SIZE_FLOAT / 2.);
        }

        self.player_camera.update(self.boundaries);

        set_camera(&self.player_camera.camera);
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let level_map = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    ];

    let mut world = World::new(level_map).await;

    loop {

        clear_background(BLACK);

        draw_text_ex("Use arrows to move player", TILE_SIZE_FLOAT * 2., TILE_SIZE_FLOAT * 1.5, TextParams::default());

        world.update();

        next_frame().await
    }
}