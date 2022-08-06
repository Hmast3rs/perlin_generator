use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Events,
    EventSettings,
    RenderEvent,
    OpenGL,
    Transformed,
    clear, rectangle
};

use perlin_gen::PerlinGen;

fn main() {
    const STEP: f64 = 1.0 / 64.0;
    const GRID_SIZE: usize = 8;
    const WINDOW_WIDTH : u32 = 400;
    const WINDOW_HEIGHT: u32 = 400;

    let sample_size : usize = (GRID_SIZE as f64 / STEP).floor() as usize;
    let size = WINDOW_WIDTH as f64 / sample_size as f64;

    let gl_version = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Noise", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(gl_version)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut noise = Vec::new();
    let (noise_sender, noise_receiver) = mpsc::channel();
    let generate_noise = move || {
        let mut generator = PerlinGen::new();
        let noise: Vec<Vec<f64>> = (0..sample_size).map(|y|
            (0..sample_size).map(|x|
                generator.get_at(x as f64 * STEP, y as f64 * STEP)
            ).collect::<Vec<f64>>()
        ).collect();
        noise_sender.send(noise).unwrap();
    };
    thread::spawn(generate_noise.clone());

    let rect = [ 0.0, 0.0, size, size ];

    let mut t1 = std::time::Instant::now();
    let mut t2;
    let mut dt;
    let mut timer = Duration::from_secs(5);
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        t2 = std::time::Instant::now();
        dt = t2 - t1;
        t1 = t2;
        timer += dt;

        if timer.as_secs() >= 5 {
            timer = Duration::ZERO;
            noise = noise_receiver.recv().unwrap();
            thread::spawn(generate_noise.clone());
        }
        
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |context, graphics, _| {
                clear([0.0; 4], graphics);
                for y in 0..sample_size {
                    for x in 0..sample_size {
                        let value = noise[y][x];
                        rectangle(
                            get_colour(value),
                            rect,
                            context.transform
                                .trans(x as f64 * size, y as f64 * size),
                            graphics,
                        );
                    } 
                }
            });
        }
    }
}

fn get_colour(value: f64) -> [f32; 4] {
    let value = value as f32;
    let w = (value + 1.0) / 2.0;
    return [w; 4];

    const BLUE : [f32; 4] = [0.0, 0.1, 0.4, 1.0];
    const LBLUE: [f32; 4] = [0.1, 0.2, 0.5, 1.0];
    const GREEN: [f32; 4] = [0.1, 0.4, 0.1, 1.0];
    const GREY : [f32; 4] = [0.7, 0.7, 0.7, 1.0];
    let partitions = [
        (0.00, BLUE),
        (0.30, LBLUE),
        (0.35, GREEN),
        (0.50, GREEN),
        (1.00, GREY),
    ];
    for i in 0..partitions.len()-1 {
        let (a, c1) = partitions[i];
        let (b, c2) = partitions[i+1];
        if w.clamp(a, b) == w {
            return colour_interpolate(c1, c2, (w - a)/(b - a));
        }
    }
    GREY
}

fn colour_interpolate(c1: [f32; 4], c2: [f32; 4], w: f32) -> [f32; 4] {
    [
        c1[0] * (1.0 - w) + c2[0] * w,
        c1[1] * (1.0 - w) + c2[1] * w,
        c1[2] * (1.0 - w) + c2[2] * w,
        c1[3] * (1.0 - w) + c2[3] * w,
    ]
}
