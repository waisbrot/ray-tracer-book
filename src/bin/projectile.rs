use std::{error::Error, io::{Stdout, self}, time::Duration, thread};

use book_renderer::tuple::{parse_vector, parse_point, Vector, Point};
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
    #[arg(long, default_value = "0,0,0", value_parser = parse_point)]
    position: Point,

    #[arg(long, default_value = "0,0,0", value_parser = parse_vector)]
    velocity: Vector,

    #[arg(long, default_value = "0,-9.8,0", value_parser = parse_vector)]
    gravity: Vector,

    #[arg(long, default_value = "0,0,0", value_parser = parse_vector)]
    wind: Vector,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let p = Projectile {
        position: args.position,
        velocity: args.velocity,
    };
    let e = Environment {
        gravity: args.gravity,
        wind: args.wind,
    };
    let mut terminal = setup_terminal()?;
    run(&mut terminal, &p, &e)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, pin: &Projectile, e: &Environment) -> Result<(), Box<dyn Error>> {
    let mut p = pin.clone();
    // let mut ticks = 0;
    while p.position.y > 0.0 {
        p = tick(&p, &e);
        // ticks += 1;
        terminal.draw(|f| {
            let data: Vec<String> = vec![p.position, p.velocity, e.gravity, e.wind].iter().map(String::from).collect();
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
        thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
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
