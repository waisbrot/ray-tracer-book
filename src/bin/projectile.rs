use std::{error::Error, io::{Stdout, self}, time::Duration, thread, fmt::Debug};

use book_renderer::{tuple::{Vector, Point}, canvas::Canvas, color};
use clap::Parser;
use ratatui::{prelude::*, Terminal, widgets::{Table, Row, Block, Borders}};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},execute};

#[derive(Debug,Clone, Copy)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl std::fmt::Display for Projectile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<⌘ {}; ↗{}>", self.position, self.velocity)
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "⫱ {}; 🍃 {}", self.gravity, self.wind)
    }
}

fn tick(projectile: &Projectile, environment: &Environment) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "0,0,0", value_parser = Point::parse_point)]
    position: Point,

    #[arg(long, default_value = "0,0,0", value_parser = Vector::parse_vector)]
    velocity: Vector,

    #[arg(long, default_value = "0,-9.8,0", value_parser = Vector::parse_vector)]
    gravity: Vector,

    #[arg(long, default_value = "0,0,0", value_parser = Vector::parse_vector)]
    wind: Vector,

    #[arg(long, default_value = "1000")]
    tick_delay: u64,

    #[arg(long, default_value = "/tmp/canvas.png")]
    outfile: String,
}

fn main() {
    let args = Args::parse();
    let p = Projectile {
        position: args.position,
        velocity: args.velocity,
    };
    let e = Environment {
        gravity: args.gravity,
        wind: args.wind,
    };
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info|{
        Simulator::restore_terminal().unwrap();
        panic_hook(info)
    }));
    let mut sim = Simulator::new(p, e, args.tick_delay, args.outfile).unwrap();
    sim.run().unwrap();
    Simulator::restore_terminal().unwrap();
}

#[derive(Debug)]
struct Bounds {
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
}

impl Bounds {
    fn new(x: i16, y: i16) -> Bounds {
        Bounds { min_x: x, min_y: y, max_x: x, max_y: y }
    }
    fn expand(&mut self, x: i16, y: i16) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }
    fn width(&self) -> usize {
        let w = self.max_x - self.min_x;
        if w < 0 {
            panic!("width is negative");
        }
        w as usize
    }
    fn height(&self) -> usize {
        let h = self.max_y - self.min_y;
        if h <= 0 {
            panic!("height is not positive: {} = {} - {}", h, self.max_y, self.min_y);
        }
        h as usize
    }

    fn position_to_canvas(&self, x: i16, y: i16) -> (usize,usize) {
        let x = x - self.min_x;
        if x < 0 {
            panic!("after subtracting min_x={}, x={} < 0", self.min_x, x);
        }
        if x > self.width() as i16 {
            panic!("x={} > width={}", x, self.width());
        }
        let y = y - self.min_y;
        if y < 0 {
            panic!("after subtracting min_y={}, y={} < 0", self.min_y, y);
        }
        if y > self.height() as i16 {
            panic!("y={} > height={}", y, self.height());
        }
        let flip_y = self.height() - y as usize;
        (x as usize, flip_y)
    }
}

struct Simulator {
    p: Projectile,
    e: Environment,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    tick_delay: u64,
    outfile: String,
}

impl Debug for Simulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Simulator")
            .field("p", &self.p)
            .field("e", &self.e)
            .field("terminal", &self.terminal.is_some())
            .field("tick_delay", &self.tick_delay)
            .field("outfile", &self.outfile)
            .finish()
    }
}

impl Simulator {
    pub fn new(p: Projectile, e: Environment, tick_delay: u64, outfile: String) -> Result<Simulator, Box<dyn Error>> {
        Ok(Simulator { p, e, tick_delay, outfile, terminal: None })
    }

    fn setup_terminal(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        self.terminal = Some(Terminal::new(CrosstermBackend::new(stdout))?);
        Ok(())
    }

    pub fn restore_terminal() -> Result<(), Box<dyn Error>> {
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(terminal.show_cursor()?)
    }

    fn find_bounds(&self) -> Result<Bounds, Box<dyn Error>> {
        let mut p = self.p.clone();
        let mut bounds = Bounds::new(p.position.x.round() as i16, p.position.y.round() as i16);
        while p.position.y > 0.0 {
            p = tick(&p, &self.e);
            bounds.expand(p.position.x.round() as i16, p.position.y.round() as i16);
        }
        Ok(bounds)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let bounds = self.find_bounds()?;
        eprintln!("Bounds={:#?}", bounds);
        let mut canvas = Canvas::new(bounds.width() + 1, bounds.height() + 1);
        // eprintln!("Canvas={:#?}", canvas);
        self.setup_terminal()?;
        while self.p.position.y > 0.0 {
            self.p = tick(&self.p, &self.e);
            let (x,y) = bounds.position_to_canvas(self.p.position.x.round() as i16, self.p.position.y.round() as i16);
            canvas.pixels[(y,x)] = color::Color::new(1.0, 0.5, 0.2);
            self.terminal.as_mut().ok_or("No terminal")?.draw(|f| {
                let data: Vec<String> = vec![self.p.position, self.p.velocity, self.e.gravity, self.e.wind].iter().map(String::from).collect();
                // let data = vec!["1,1","2,2","3,3","4,4"];
                let data_rows = vec![Row::new(data)];
                let header = Row::new(vec!["Position", "Velocity", "Gravity", "Wind"]).style(Style::default().fg(Color::Yellow)).bottom_margin(1);
                let block = Block::default()
                    .title("Projectile")
                    .borders(Borders::ALL);
                let table = Table::new(data_rows).header(header).block(block).widths(&[
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]);
    
                let size = f.size();
                f.render_widget(table, size);
            })?;
            thread::sleep(Duration::from_millis(self.tick_delay));
        }
        canvas.write_png(&self.outfile)?;
        Ok(())
    }
}




    //     println!("Starting out. Projectile: {}; Environment: {}", &p, &e);
//     println!("After {} ticks: {}", &ticks, &p);

//     Ok(loop {
//         terminal.draw(|frame| {
//             let greeting = Paragraph::new("Hello World!");
//             frame.render_widget(greeting, frame.size());
//         })?;
//         if event::poll(Duration::from_millis(250))? {
//             if let Event::Key(key) = event::read()? {
//                 if KeyCode::Char('q') == key.code {
//                     break;
//                 }
//             }
//         }
//     })
// }


// //     println!("Starting out. Projectile: {}; Environment: {}", &p, &e);
// //     let mut ticks = 0;
// //     while p.position.y > 0.0 {
// //         p = tick(&p, &e);
// //         ticks += 1;
// //     }
// //     println!("After {} ticks: {}", &ticks, &p);
// // }
