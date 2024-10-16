use glam::Vec2;
use rand::prelude::*;
use winit::dpi::PhysicalPosition;

use ggez::{
    conf::WindowMode,
    event::{ 
        self, 
        EventHandler 
    }, 
    graphics:: {
        self, Color, DrawParam, Mesh, MeshBuilder, Rect, Text
    }, 
    input::{
        keyboard::KeyCode, mouse::MouseButton
    },
    Context, 
    ContextBuilder, 
    GameResult, 
    *,
};
fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
            .window_mode(WindowMode::default()
                .dimensions(2300.0, 2000.0)
                .borderless(true)
            )
        .build()
        .expect("aieee, could not create ggez context!");
    
    // Create an instance of your event handler. 
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = State::new(&mut ctx, 3000, 20, /*2,*/ 0.040, 0.05).unwrap();
    

    /*std::thread::spawn(move || {
        event::run(ctx2, event_loop2, my_game2);
    }); */   
    // Run!
    event::run(ctx, event_loop, my_game);

}

#[derive(Debug)]
struct State {
    n: u32, // number of particles 
    n_colours: u8, // number of colours
//    n_d: u8, // number of dimensions

    particles: Vec<Particle>,

    attraction_matrix: Vec<Vec<f32>>,

    f_halflife: f32,
    f_factor: f32,
    r_max: f32,
    fo_factor: f32,

    dt: f32,

    am_m: Mesh,

    window_drag_offset: (f32, f32),

    cls: Vec<Color>
    /*positions: Vec<(f32, f32)>
    velocities: */
}

#[derive(Debug)]
struct Particle {
    pos: (f32, f32),
    vel: (f32, f32),
    color: u8
}

impl State {
    fn new(ctx: &mut Context, n: u32, n_colours: u8, /*n_d: u8,*/ f_halflife: f32, r_max: f32) -> GameResult<Self> {
        /*let s_a = 1.0;
        let p_a = 0.0;
        let p2_a = 0.0;
        //let m_a = 0.0;
        let n2_a = 0.0;
        let n_a = 0.5;*/

        //let attraction_matrix = State::randomise_matrix(n_colours);
        let attraction_matrix = State::generate_snake_matrix(1.0, 0.5, 0.0, n_colours);

        let mut cls: Vec<Color> = vec![];
        let angle = 360.0 / n_colours as f32;
        for i in 0..n_colours {
            cls.push(hsv_to_rgb(i as f32 * angle, 1.0, 1.0))
        }
        
        /*let attraction_matrix = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.5, 0.0],
            vec![1.0, -1.0, 0.0],
        ];*/
        /*let attraction_matrix= vec![
            vec![s_a, n_a, n2_a, 0.0f32, 0.0f32, 0.0f32, p2_a, p_a],
            vec![p_a, s_a, n_a, n2_a, 0.0f32, 0.0f32, 0.0f32, p2_a],
            vec![p2_a, p_a, s_a, n_a, n2_a, 0.0f32, 0.0f32, 0.0f32],
            vec![0.0f32, p2_a, p_a, s_a, n_a, n2_a, 0.0f32, 0.0f32],
            vec![0.0f32, 0.0f32, p2_a, p_a, s_a, n_a, n2_a, 0.0f32],
            vec![0.0f32, 0.0f32, 0.0f32, p2_a, p_a, s_a, n_a, n2_a],
            vec![n2_a, 0.0f32, 0.0f32, 0.0f32, p2_a, p_a, s_a, n_a],
            vec![n_a, 0.0f32, 0.0f32, 0.0f32, 0.0f32, p2_a, p_a, s_a],
        ];*/
        /*let attraction_matrix= vec![
            vec![s_a, n_a, 0.0f32, 0.0f32, 0.0f32, 0.0f32,],
            vec![0.0f32, s_a, n_a, 0.0f32, 0.0f32, 0.0f32,],
            vec![0.0f32, 0.0f32, s_a, n_a, 0.0f32, 0.0f32,],
            vec![0.0f32, 0.0f32, 0.0f32, s_a, n_a, 0.0f32,],
            vec![0.0f32, 0.0f32, 0.0f32, 0.0f32, s_a, n_a,],
            vec![n_a, 0.0f32, 0.0f32, 0.0f32, 0.0f32, s_a,],
        ];*/

        let mut am_mb = MeshBuilder::new();
        am_mb.rectangle(graphics::DrawMode::fill(), Rect::new(2000.0, 0.0, 300.0, 300.0), Color::from_rgb(100, 100, 100))?;
        for (i, item) in attraction_matrix.iter().enumerate() {
            for (j, &jtem) in item.iter().enumerate() {
                if jtem >= 0.0 {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(2005.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb(0, (255.0 * jtem) as u8, 0)
                    )?;
                } else {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(2005.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb((255.0 * jtem.abs()) as u8, 0, 0)
                    )?;
                }
            }
        }
        let am_m = Mesh::from_data(ctx, am_mb.build());
        Ok(State {
            n,
            n_colours,
//            n_d,
            particles: State::randomise_particles(n, n_colours),
            attraction_matrix,
            f_halflife,
            f_factor: 0.5f32.powf(0.01 / f_halflife),
            r_max,
            fo_factor: 20.0,
            dt: 0.01,
            am_m,
            window_drag_offset: (0.0, 0.0),
            cls
        })
    }

    fn _randomise_matrix(n_colours: u8) -> Vec<Vec<f32>> {
        let mut rng = rand::thread_rng();
        let mut matrix: Vec<Vec<f32>> = vec![];
        for _i in 0..n_colours {
            let mut row: Vec<f32> = vec![];
            for _j in 0..n_colours {
                row.push(rng.gen::<f32>() * 2.0 - 1.0);
            }
            matrix.push(row);
        }
        println!("{:#?}", matrix);
        matrix
    }

    fn randomise_particles(n: u32, n_colours: u8) -> Vec<Particle> {
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = vec![];
        for _i in 0..n {
            let mut p = Particle::new();
            p.pos = (rng.gen::<f32>(), rng.gen::<f32>());
            p.color = (rng.gen::<f32>() * n_colours as f32).floor() as u8;
            if p.color == n_colours - 1 { println!("nvm")}
            println!("{:?}", p.color);
            particles.push(p);
        }
        particles
    }

    fn generate_snake_matrix(s_a: f32, n_a: f32, p_a: f32, n_colours: u8) -> Vec<Vec<f32>> {
        let mut matrix: Vec<Vec<f32>> = vec![];

        for i in 0..n_colours as usize {
            let mut row: Vec<f32> = vec![];
            for _j in 0..n_colours as usize {
                row.push(0.0);
            }
            row[i] = s_a;
            row[(i + 1) % n_colours as usize] = n_a;
            row[((i as isize - 1).rem_euclid(n_colours as isize)) as usize] = p_a;
            matrix.push(row);
        };

        matrix
    }

    #[inline]
    fn calculate_force(r: f32, a: f32) -> f32 {
        let beta = 0.3;
        if r < beta {
            r / beta - 1.0
        } else if beta < r && r < 1.0 {
            a * (1.0 - (2.0 * r - 1.0 - beta).abs() / (1.0 - beta))
        } else {0.0}
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let m_ctx = &ctx.mouse;
        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_just_pressed(KeyCode::Period) {
            self.fo_factor = match self.fo_factor {
                0.3 => 0.5,
                0.5 => 0.75,
                0.75 => 1.0,
                1.0 => 1.5,
                1.5 => 2.0,
                2.0 => 3.0,
                3.0 => 4.0,
                4.0 => 5.0,
                5.0 => 7.5,
                7.5 => 10.0,
                10.0 => 12.5,
                12.5 => 17.5,
                17.5 => 25.0,
                25.0 => 50.0,
                _ => 50.0
            };
        }
        if k_ctx.is_key_just_pressed(KeyCode::Comma) {
            self.fo_factor = match self.fo_factor {
                0.5 => 0.3,
                0.75 => 0.5,
                1.0 => 0.75,
                1.5 => 1.0,
                2.0 => 1.5,
                3.0 => 2.0,
                4.0 => 3.0,
                5.0 => 4.0,
                7.5 => 5.0,
                10.0 => 7.5,
                12.5 => 10.0,
                17.5 => 12.5,
                25.0 => 17.5,
                50.0 => 25.0,
                _ => 0.3
            };
        }
        /*if k_ctx.is_key_just_pressed(KeyCode::Equals) {
            
        }*/

        if m_ctx.button_pressed(MouseButton::Left) {
            let current_pos = ctx.gfx.window_position().unwrap();
            let pos = PhysicalPosition::new(current_pos.x + m_ctx.delta().x as i32 + self.window_drag_offset.0 as i32, current_pos.y + m_ctx.delta().y as i32 + self.window_drag_offset.1 as i32);
            self.window_drag_offset = (m_ctx.delta().x + self.window_drag_offset.0, m_ctx.delta().y + self.window_drag_offset.1);
            ctx.gfx.set_window_position(pos)?;
        }
        
        let f_fact = self.r_max * self.fo_factor * self.dt;
        for i in 0..self.n as usize {
            let col_usize = self.particles[i].color as usize;
            let mut total_fx = 0.0;
            let mut total_fy = 0.0;
    
            for j in 0..self.n as usize {
                if j == i { continue; }
                let rx = self.particles[j].pos.0 - self.particles[i].pos.0;
                let ry = self.particles[j].pos.1 - self.particles[i].pos.1;
                let r = (rx.powi(2) + ry.powi(2)).sqrt();
                if r > 0.0 && r < self.r_max {
                    let f = State::calculate_force(r / self.r_max, self.attraction_matrix[col_usize][self.particles[j].color as usize]);
                    total_fx += rx / r * f;
                    total_fy += ry / r * f;
                }
            }    
            total_fx *= f_fact;
            total_fy *= f_fact;
            self.particles[i].vel.0 = self.particles[i].vel.0 * self.f_factor + total_fx;
            self.particles[i].vel.1 = self.particles[i].vel.1 * self.f_factor + total_fy;
        }
        for i in 0..self.n as usize {
            self.particles[i].pos.0 = (self.particles[i].pos.0 + self.particles[i].vel.0 * self.dt).clamp(0.0, 1.0);
            self.particles[i].pos.1 = (self.particles[i].pos.1 + self.particles[i].vel.1 * self.dt).clamp(0.0, 1.0); 
            self.particles[i].vel.0 = if self.particles[i].pos.0 == 0.0 || self.particles[i].pos.0 == 1.0 { 0.0 } else { self.particles[i].vel.0 };
            self.particles[i].vel.1 = if self.particles[i].pos.1 == 0.0 || self.particles[i].pos.1 == 1.0 { 0.0 } else { self.particles[i].vel.1 };
        }
        Ok(())
    }
        
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        let mut mb = MeshBuilder::new();
        
        /*let cls = vec![
            Color::from_rgb(255, 48, 238),
            Color::from_rgb(255, 74, 240),
            Color::from_rgb(255, 117, 244),
            Color::from_rgb(255, 158, 247),
            Color::from_rgb(255, 191, 250),
            Color::from_rgb(255, 217, 252),
            Color::from_rgb(255, 235, 253),
            Color::from_rgb(255, 252, 255),
        ];*/
        /*let cls = vec![
            Color::from_rgb(255, 0, 0),
            Color::from_rgb(245, 150, 0),
            Color::from_rgb(245, 235, 0),
            Color::from_rgb(0, 245, 15),
            Color::from_rgb(0, 135, 245),
            Color::from_rgb(165, 0, 245),
        ];*/

        for i in 0..self.n as usize {
            mb.circle(
                graphics::DrawMode::fill(), 
                Vec2::new(self.particles[i].pos.0 * 2000.0, self.particles[i].pos.1 * 2000.0), 
                10.0,
                1.0, 
                self.cls[self.particles[i].color as usize]
            )?;
        }
        
        let mut text1 = Text::new(format!("# of Particles: {:?}", self.n));
        text1.set_scale(25.0);
        let mut text2 = Text::new(format!("# of Colours: {:?}", self.n_colours));
        text2.set_scale(25.0);
        let mut text3 = Text::new(format!("Attraction Radius: {:?}", self.r_max));
        text3.set_scale(25.0);
        let mut text4 = Text::new(format!("Force Factor: {:?}", self.fo_factor));
        text4.set_scale(25.0);
        let mut text5 = Text::new(format!("Friction Half-Life: {:?}", self.f_halflife));
        text5.set_scale(25.0);
        let mut text6 = Text::new(format!("FPS: {:?}", ctx.time.fps()));
        text6.set_scale(25.0);

        canvas.draw(&Mesh::from_data(ctx, mb.build()), DrawParam::default());
        canvas.draw(&self.am_m, DrawParam::default());

        canvas.draw(&text1, DrawParam::default().dest(Vec2::new(30.0, 30.0)).color(Color::WHITE));
        canvas.draw(&text2, DrawParam::default().dest(Vec2::new(30.0, 60.0)).color(Color::WHITE));
        canvas.draw(&text3, DrawParam::default().dest(Vec2::new(30.0, 90.0)).color(Color::WHITE));
        canvas.draw(&text4, DrawParam::default().dest(Vec2::new(30.0, 120.0)).color(Color::WHITE));
        canvas.draw(&text5, DrawParam::default().dest(Vec2::new(30.0, 150.0)).color(Color::WHITE));
        canvas.draw(&text6, DrawParam::default().dest(Vec2::new(30.0, 180.0)).color(Color::WHITE));
        // Draw code here...
        canvas.finish(ctx)
    }
}

impl Particle {
    fn new() -> Particle {
        Particle {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            color: 0
        }
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    Color::from_rgb(r, g, b)
}