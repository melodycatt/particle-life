use glam::Vec2;
use rand::prelude::*;
use winit::dpi::PhysicalPosition;
use std::io::Write;

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
        keyboard::KeyCode, mouse::MouseButton, keyboard::KeyMods
    },
    Context, 
    ContextBuilder, 
    GameResult, 
    *,
};
fn main() {
    //println!("{:#?}", args);
    let mut particles_in = String::new();
    print!("# of Particles (default = 3000): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut particles_in).expect("poo");
    println!("{:?}", particles_in);
    let particles: u32 = if particles_in == "\n" || particles_in == "\r\n" { 3000 } else { particles_in.trim().parse().expect("not a number!") }; 
    let mut colours_in = String::new();
    print!("# of Colours (default = 6): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut colours_in).expect("poo");
    let colours: u8 = if colours_in == "\n" || colours_in == "\r\n" { 6 } else { colours_in.trim().parse().expect("not a number!") }; 
    let mut rmax_in = String::new();
    print!("Attraction radius (default = 0.1): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut rmax_in).expect("poo");
    let rmax: f32 = if rmax_in == "\n" || rmax_in == "\r\n" { 0.1 } else { rmax_in.trim().parse().expect("not a number!") }; 
    let mut fhl_in = String::new();
    print!("Friction Half-Life (default = 0.040): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut fhl_in).expect("poo");
    let fhl: f32 = if fhl_in == "\n" || fhl_in == "\r\n" { 0.04 } else { fhl_in.trim().parse().expect("not a number!") }; 
    let mut dim_in = String::new();
    print!("Width and height (square window, default = 2000.0): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut dim_in).expect("poo");
    let dim: f32 = if dim_in == "\n" || dim_in == "\r\n" { 0.04 } else { dim_in.trim().parse().expect("not a number!") }; 
    let mut snake = String::new();
    print!("Snake? (y/n): ");
    std::io::stdout().flush().expect("poo");
    std::io::stdin().read_line(&mut snake).expect("poo");
    
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
            .window_mode(WindowMode::default()
                .dimensions(dim + 300.0, dim)
                .borderless(true)
            )
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = if snake == "y\n" || snake == "y\r\n" { State::new_snake(&mut ctx, particles, colours, /*2,*/ fhl, rmax, 0.01, dim).unwrap() }
                            else { State::new(&mut ctx, particles, colours, /*2,*/ fhl, rmax, 0.0125, dim).unwrap() };
    
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

    cls: Vec<Color>,

    dim: f32,

    chemistry_matrix: Vec<Vec<u8>>,
    reaction_r_max: f32
//    quit: bool,
    /*positions: Vec<(f32, f32)>
    velocities: */
}

#[derive(Debug, Clone)]
struct Particle {
    pos: (f32, f32),
    vel: (f32, f32),
    color: u8,
    id: u32
}

impl State {
    fn new(ctx: &mut Context, n: u32, n_colours: u8, /*n_d: u8,*/ f_halflife: f32, r_max: f32, reaction_r_max: f32, dim: f32) -> GameResult<Self> {
        let (attraction_matrix, chemistry_matrix) = State::randomise_matrix(n_colours);
        //let attraction_matrix = State::generate_snake_matrix(1.0, 0.5, 0.0, n_colours);

        let mut cls: Vec<Color> = vec![];
        let angle = 360.0 / n_colours as f32;
        for i in 0..n_colours {
            cls.push(hsv_to_rgb(i as f32 * angle, 1.0, 1.0))
        }
        
        let mut am_mb = MeshBuilder::new();
        am_mb.rectangle(graphics::DrawMode::fill(), Rect::new(dim, 0.0, 300.0, 300.0), Color::from_rgb(100, 100, 100))?;
        for (i, item) in attraction_matrix.iter().enumerate() {
            am_mb.rectangle(
                graphics::DrawMode::fill(), 
                Rect::new(dim - 55.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                cls[i]
            )?;
            for (j, &jtem) in item.iter().enumerate() {
                if jtem >= 0.0 {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(dim + 5.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb(0, (255.0 * jtem) as u8, 0)
                    )?;
                } else {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(dim + 05.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb((255.0 * jtem.abs()) as u8, 0, 0)
                    )?;
                }
            }
        }
        am_mb.rectangle(graphics::DrawMode::fill(), Rect::new(dim, 350.0, 300.0, 300.0), Color::from_rgb(100, 100, 100))?;
        for (i, item) in chemistry_matrix.iter().enumerate() {
            am_mb.rectangle(
                graphics::DrawMode::fill(), 
                Rect::new(dim - 55.0, 355.0 + i as f32 * 50.0, 40.0, 40.0), 
                cls[i]
            )?;
            for (j, &jtem) in item.iter().enumerate() {
                am_mb.rectangle(
                    graphics::DrawMode::fill(), 
                    Rect::new(dim + 5.0 + j as f32 * 50.0, 355.0 + i as f32 * 50.0, 40.0, 40.0), 
                    cls[jtem as usize]
                )?;
            }
        }
        let am_m = Mesh::from_data(ctx, am_mb.build());
        Ok(State {
            n,
            n_colours,
            particles: State::randomise_particles(n, n_colours),
            chemistry_matrix,
            attraction_matrix,
            f_halflife,
            f_factor: 0.5f32.powf(0.01 / f_halflife),
            r_max,
            fo_factor: 10.0,
            dt: 0.01,
            am_m,
            window_drag_offset: (0.0, 0.0),
            cls,
            dim,
            reaction_r_max,
        })
    }

    fn new_snake(ctx: &mut Context, n: u32, n_colours: u8, /*n_d: u8,*/ f_halflife: f32, r_max: f32, reaction_r_max: f32, dim: f32) -> GameResult<Self> {
        let chemistry_matrix = State::randomise_matrix(n_colours).1;
        let attraction_matrix = State::generate_snake_matrix(1.0, 0.5, 0.0, n_colours);

        let mut cls: Vec<Color> = vec![];
        let angle = 360.0 / n_colours as f32;
        for i in 0..n_colours {
            cls.push(hsv_to_rgb(i as f32 * angle, 1.0, 1.0))
        }
        
        let mut am_mb = MeshBuilder::new();
        am_mb.rectangle(graphics::DrawMode::fill(), Rect::new(dim, 0.0, 300.0, 300.0), Color::from_rgb(100, 100, 100))?;
        for (i, item) in attraction_matrix.iter().enumerate() {
            for (j, &jtem) in item.iter().enumerate() {
                if jtem >= 0.0 {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(dim + 5.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb(0, (255.0 * jtem) as u8, 0)
                    )?;
                } else {
                    am_mb.rectangle(
                        graphics::DrawMode::fill(), 
                        Rect::new(dim + 5.0 + j as f32 * 50.0, 5.0 + i as f32 * 50.0, 40.0, 40.0), 
                        Color::from_rgb((255.0 * jtem.abs()) as u8, 0, 0)
                    )?;
                }
            }
        }
        am_mb.rectangle(graphics::DrawMode::fill(), Rect::new(dim, 350.0, 300.0, 300.0), Color::from_rgb(100, 100, 100))?;
        for (i, item) in chemistry_matrix.iter().enumerate() {
            for (j, &jtem) in item.iter().enumerate() {
                am_mb.rectangle(
                    graphics::DrawMode::fill(), 
                    Rect::new(dim + 5.0 + j as f32 * 50.0, 355.0 + i as f32 * 50.0, 40.0, 40.0), 
                    cls[jtem as usize]
                )?;
            }
        }
        let am_m = Mesh::from_data(ctx, am_mb.build());
        Ok(State {
            n,
            n_colours,
            particles: State::randomise_particles(n, n_colours),
            attraction_matrix,
            chemistry_matrix,
            f_halflife,
            f_factor: 0.5f32.powf(0.01 / f_halflife),
            r_max,
            fo_factor: 10.0,
            dt: 0.01,
            am_m,
            window_drag_offset: (0.0, 0.0),
            cls,
            dim,
            reaction_r_max,
//            quit: false,
        })
    }

    fn randomise_matrix(n_colours: u8) -> (Vec<Vec<f32>>, Vec<Vec<u8>>) {
        let mut rng = rand::thread_rng();
        let mut matrix: Vec<Vec<f32>> = vec![];
        let mut c_matrix: Vec<Vec<u8>> = vec![];
        for _i in 0..n_colours {
            let mut row: Vec<f32> = vec![];
            let mut c_row: Vec<u8> = vec![];
            for _j in 0..n_colours {
                row.push(rng.gen::<f32>() * 2.0 - 1.0);
                c_row.push(rng.gen_range(0..n_colours));
            }
            matrix.push(row);
            c_matrix.push(c_row);
        }
        (matrix, c_matrix)
    }

    fn randomise_particles(n: u32, n_colours: u8) -> Vec<Particle> {
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = vec![];
        for i in 0..n {
            let mut p = Particle::new(i);
            p.pos = (rng.gen::<f32>(), rng.gen::<f32>());
            p.color = (rng.gen::<f32>() * n_colours as f32).floor() as u8;
            particles.push(p);
        }
        particles
    }

    fn add_particles(&self, n: u32, n_colours: u8) -> Vec<Particle> {
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = vec![];
        for i in self.n..(self.n + n) {
            let mut p = Particle::new(i);
            p.pos = (rng.gen::<f32>(), rng.gen::<f32>());
            p.color = (rng.gen::<f32>() * n_colours as f32).floor() as u8;
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

    #[inline(always)]
    fn calculate_force(r: f32, a: f32) -> f32 {
        let beta = 0.1;
        if r < beta {
            r / beta - 1.0
        } else if beta < r && r < 1.0 {
            a * (1.0 - (2.0 * r - 1.0 - beta).abs() / (1.0 - beta))
        } else {0.0}
    }

    #[inline(always)]
    fn handle_keys(&mut self, ctx: &mut Context) -> Result<(), GameError> {
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

        if k_ctx.is_key_just_pressed(KeyCode::Minus) && self.n > 0 {
            self.particles.truncate((self.n - if k_ctx.is_mod_active(KeyMods::SHIFT) { 50 } else { 150 }) as usize);
            self.n -= if k_ctx.is_mod_active(KeyMods::SHIFT) { 50 } else { 150 };
        }
        if k_ctx.is_key_just_pressed(KeyCode::Equals) {
            let mut new_particles = self.add_particles(if k_ctx.is_mod_active(KeyMods::SHIFT) { 50 } else { 150 }, self.n_colours);
            self.particles.append(&mut new_particles);
            self.n += if k_ctx.is_mod_active(KeyMods::SHIFT) { 50 } else { 150 };
        }

        if m_ctx.button_pressed(MouseButton::Left) {
            let current_pos = ctx.gfx.window_position().unwrap();
            let pos = PhysicalPosition::new(current_pos.x + m_ctx.delta().x as i32 + self.window_drag_offset.0 as i32, current_pos.y + m_ctx.delta().y as i32 + self.window_drag_offset.1 as i32);
            self.window_drag_offset = (m_ctx.delta().x + self.window_drag_offset.0, m_ctx.delta().y + self.window_drag_offset.1);
            ctx.gfx.set_window_position(pos)?;
        }

        Ok(())
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.handle_keys(ctx)?;
        
        let f_fact = self.r_max * self.fo_factor * self.dt;
        //let iterp = self.particles.iter();
        let check_vec = self.particles.clone();
        for particle in self.particles.iter_mut() {
            let col_usize = particle.color as usize;
            let mut total_fx = 0.0;
            let mut total_fy = 0.0;
            
            for check in check_vec.iter() {
                if check.id == particle.id { continue; }
                let rx = check.pos.0 - particle.pos.0;
                let ry = check.pos.1 - particle.pos.1;
                let r = (rx.powi(2) + ry.powi(2)).sqrt();
                if r > 0.0 && r < self.reaction_r_max {
                    particle.color = self.chemistry_matrix[col_usize][check.color as usize];
                    //self.particles[self.particles.iter().position(|x| x.id == check.id).unwrap()].color = self.chemistry_matrix[check.color as usize][col_usize];
                    total_fx = rx / r * -10.0;
                    total_fy = ry / r * -10.0;
                    break;
                }
                else if r > 0.0 && r < self.r_max {
                    let f = State::calculate_force(r / self.r_max, self.attraction_matrix[col_usize][check.color as usize]);
                    total_fx += rx / r * f;
                    total_fy += ry / r * f;
                }
            }    
            
            total_fx *= f_fact;
            total_fy *= f_fact;
            particle.vel.0 = particle.vel.0 * self.f_factor + total_fx;
            particle.vel.1 = particle.vel.1 * self.f_factor + total_fy;
        }
        for particle in self.particles.iter_mut() {
            particle.pos.0 +=  particle.vel.0 * self.dt;
            particle.pos.1 +=  particle.vel.1 * self.dt;

            //// CLAMP
            /*
            particle.pos.0 = (particle.pos.0 + particle.vel.0 * self.dt).clamp(0.0, 1.0);
            particle.pos.1 = (particle.pos.1 + particle.vel.1 * self.dt).clamp(0.0, 1.0);
            particle.vel.0 = if particle.pos.0 == 0.0 || particle.pos.0 == 1.0 { 0.0 } else { particle.vel.0 };
            particle.vel.1 = if particle.pos.1 == 0.0 || particle.pos.1 == 1.0 { 0.0 } else { particle.vel.1 };
            */
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
                Vec2::new(self.particles[i].pos.0 * self.dim, self.particles[i].pos.1 * self.dim), 
                10.0 * (self.dim / 2000.0),
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
        let mut text7 = Text::new(format!("Use < and > to speed up and slow down the sim"));
        text7.set_scale(25.0);
        let mut text8 = Text::new(format!("Use + and - to add and remove particles, use shift for precise"));
        text8.set_scale(25.0);
        /*let mut text9 = Text::new(format!("R to restart"));
        text9.set_scale(25.0);*/

        canvas.draw(&Mesh::from_data(ctx, mb.build()), DrawParam::default());
        canvas.draw(&self.am_m, DrawParam::default());

        canvas.draw(&text1, DrawParam::default().dest(Vec2::new(30.0, 30.0)).color(Color::WHITE));
        canvas.draw(&text2, DrawParam::default().dest(Vec2::new(30.0, 60.0)).color(Color::WHITE));
        canvas.draw(&text3, DrawParam::default().dest(Vec2::new(30.0, 90.0)).color(Color::WHITE));
        canvas.draw(&text4, DrawParam::default().dest(Vec2::new(30.0, 120.0)).color(Color::WHITE));
        canvas.draw(&text5, DrawParam::default().dest(Vec2::new(30.0, 150.0)).color(Color::WHITE));
        canvas.draw(&text6, DrawParam::default().dest(Vec2::new(30.0, 180.0)).color(Color::WHITE));
        canvas.draw(&text7, DrawParam::default().dest(Vec2::new(30.0, 210.0)).color(Color::WHITE));
        canvas.draw(&text8, DrawParam::default().dest(Vec2::new(30.0, 240.0)).color(Color::WHITE));
        //canvas.draw(&text9, DrawParam::default().dest(Vec2::new(30.0, 1960.0)).color(Color::WHITE));
        // Draw code here...
        canvas.finish(ctx)
    }

}

impl Particle {
    fn new(id: u32) -> Particle {
        Particle {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            color: 0,
            id
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