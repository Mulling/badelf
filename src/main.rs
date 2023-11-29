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

struct Tui {
    stdout: Stdout,
    data: Vec<String>,
    x: u16,
    y: u16,
    cx: u16,
    cy: u16,
    line: u16,
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            stdout: stdout(),
            data: vec![],
            x: 0,
            y: 0,
            cx: 0,
            cy: 0,
            line: 0,
        }
    }
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

    fn run(&mut self) -> Result<(), Error> {
        let mut quit = false;

        for (i, l) in self.data.iter().take(self.y.into()).enumerate() {
            self.stdout.execute(MoveTo(0, i as u16))?;
            self.stdout.execute(Print(l))?;
        }

        while !quit {
            match read()? {
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        if let KeyCode::Char(c) = event.code {
                            match c {
                                'j' => {
                                    if self.cy == self.y - 1 {
                                        self.line += 1;
                                        let start: usize = (self.line - self.cy).into();

                                        self.stdout.execute(Clear(terminal::ClearType::All))?;
                                        for (i, l) in
                                            self.data[start..self.line.into()].iter().enumerate()
                                        {
                                            self.stdout.execute(MoveTo(0, i as u16))?;
                                            self.stdout.execute(Print(l))?;
                                        }
                                    } else {
                                        self.line += 1;
                                        self.cy += 1;
                                    }
                                }
                                'k' => {
                                    if self.cy == 0 {
                                        if self.line != 0 {
                                            self.line -= 1;

                                            let end: usize = (self.line + self.y).into();
                                            self.stdout.execute(Clear(terminal::ClearType::All))?;

                                            for (i, l) in
                                                self.data[self.line.into()..end].iter().enumerate()
                                            {
                                                self.stdout.execute(MoveTo(0, i as u16))?;
                                                self.stdout.execute(Print(l))?;
                                            }
                                        }
                                    } else {
                                        if self.line != 0 {
                                            self.line -= 1;
                                        }
                                        self.cy -= 1;
                                    }
                                }
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
