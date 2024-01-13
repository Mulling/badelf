use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEventKind},
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use std::{
    fs,
    io::{stdout, Error, Stdout},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("a.out"))]
    file: String,
}

#[derive(Debug)]
struct Tui {
    stdout: Stdout,
    data: Vec<String>,
    x: u16,
    y: u16,
    cx: u16,
    cy: u16,
    line: usize,
}

impl Tui {
    fn new(file: String) -> Result<Self, Error> {
        let (x, y) = terminal::size()?;

        let data = fs::read_to_string(file)?
            .lines()
            .map(|l| l.to_string())
            .collect();

        let mut tui = Tui {
            stdout: stdout(),
            data,
            x,
            y,
            cx: 0,
            cy: 0,
            line: 0,
        };
        tui.stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        tui.stdout.execute(MoveTo(tui.cx, tui.cy))?;
        Ok(tui)
    }

    fn move_cursor(&mut self, x: u16, y: u16) -> Result<(), Error> {
        self.stdout.execute(MoveTo(x, y))?;
        Ok(())
    }

    fn scroll_down(&mut self) -> Result<(), Error> {
        self.line = self.data.len().min(self.line + 1);

        if self.cy == self.y - 1 {
            let start: usize = self.line - self.cy as usize;
            let end: usize = self.data.len().min(self.line);
            self.draw(start, end)?;
        } else {
            self.cy += 1;
        }
        Ok(())
    }

    fn scroll_up(&mut self) -> Result<(), Error> {
        self.line = self.line.saturating_sub(1);

        if self.cy == 0 {
            let end: usize = self.line + self.y as usize;

            self.draw(self.line, end)?;
        } else {
            self.cy -= 1;
        }

        Ok(())
    }

    fn draw(&mut self, start: usize, end: usize) -> Result<(), Error> {
        self.stdout.execute(Clear(terminal::ClearType::All))?;

        for (i, l) in (start..end).enumerate() {
            self.stdout.execute(MoveTo(0, i as u16))?;
            self.stdout.execute(Print(&self.data[l]))?;
        }
        Ok(())
    }

    fn run(&mut self) -> Result<(), Error> {
        let mut quit = false;

        self.draw(0, self.y.into())?;
        self.move_cursor(0, 0)?;

        while !quit {
            match read()? {
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        if let KeyCode::Char(c) = event.code {
                            match c {
                                'j' => self.scroll_down()?,
                                'k' => self.scroll_up()?,
                                'q' => quit = true,
                                _ => (),
                            }
                        }
                    }
                }
                Event::Resize(x, y) => {
                    self.x = x;
                    self.y = y;
                }
                _ => (),
            }
            self.stdout.execute(MoveTo(self.cx, self.cy))?;
        }
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = self.stdout.execute(LeaveAlternateScreen);
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    Tui::new(args.file)?.run()?;

    Ok(())
}
