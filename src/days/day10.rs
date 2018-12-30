extern crate regex;
extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;

use self::regex::Regex;

use std::error::Error;
use std::fs;
use std::path::Path;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
static WINDOWS_WIDTH: u32 = 800;
static WINDOWS_HEIGHT: u32 = 600;

#[derive(Debug)]
struct Light {
    rect: Rect,
    vx: i32,
    vy: i32,
}

impl Light {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Light {
            vx,
            vy,
            rect: Rect::new(x, y, 2, 2),
        }
    }

    fn translocate(&mut self, speed: i32) {
        let (x, y) = (self.rect.x(), self.rect.y());
        self.rect.set_x(x + (speed * self.vx));
        self.rect.set_y(y + (speed * self.vy));
    }

    fn draw(&self, canvas: &mut Canvas) {
        if let Err(reason) = canvas.draw_rect(self.rect) {
            println!("{}", reason);
        }
    }
}

fn get_input() -> Vec<Light> {
    let input = fs::read_to_string(Path::new("../data/day10.txt")).unwrap();
    let reg = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut lights = Vec::<Light>::new();

    for cap in reg.captures_iter(&input) {
        let (x, y, vx, vy) = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
        lights.push(Light::new(x, y, vx, vy));
    }

    lights
}

fn init_sdl2() -> (Canvas, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOWS_WIDTH, WINDOWS_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    (canvas, sdl_context.event_pump().unwrap())
}

fn fast_forward(lights: &mut [Light]) -> i32 {
    let mut time = 0;
    loop {
        {
            let light = &lights[0];
            let (x, y) = (light.rect.x(), light.rect.y());
            if x >= 0 && y >= 0 && x < WINDOWS_WIDTH as i32 && y < WINDOWS_HEIGHT as i32 {
                break;
            }
        }
        for light in lights.iter_mut() {
            light.translocate(1);
        }
        time += 1;
    }
    time
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut lights = get_input();
    let (mut canvas, mut event_pump) = init_sdl2();
    let mut time = fast_forward(&mut lights);

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let mut speed = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    speed = 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    speed = -10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    speed = -1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    speed = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("Current time: {}", time);
                }
                _ => {}
            }
        }
        time += speed;
        for light in lights.iter_mut() {
            light.translocate(speed);
            light.draw(&mut canvas);
        }
        canvas.present();
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    Ok(())
}
