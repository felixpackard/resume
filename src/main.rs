use std::io;

use ::image::DynamicImage;
use image::fetch_image;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::ListState,
    DefaultTerminal, Frame,
};

pub mod image;
pub mod json;
pub mod ui;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    should_exit: bool,
    resume: Resume,
    pages: Pages,
}

#[derive(Debug)]
struct Resume {
    data: ResumeSchema,
    cached_image: Option<DynamicImage>,
}

impl From<ResumeSchema> for Resume {
    fn from(data: ResumeSchema) -> Self {
        let mut cached_image = None;

        if let Some(basics) = data.basics.as_ref() {
            if let Some(src) = basics.image.as_ref() {
                cached_image = match fetch_image(src) {
                    Ok(img) => Some(img),
                    Err(_) => None,
                }
            }
        }

        Self { data, cached_image }
    }
}

#[derive(Debug, Clone, Copy)]
enum PageType {
    Overview,
    Work,
    Education,
    Skills,
    Interests,
    Languages,
    Portrait,
}

impl PageType {
    fn label(&self) -> &str {
        match self {
            Self::Overview => "Overview",
            Self::Work => "Work",
            Self::Education => "Education",
            Self::Skills => "Skills",
            Self::Interests => "Interests",
            Self::Languages => "Languages",
            Self::Portrait => "Surprise!",
        }
    }
}

#[derive(Debug, Default)]
struct Pages {
    items: Vec<PageType>,
    state: ListState,
}

impl From<&ResumeSchema> for Pages {
    fn from(resume: &ResumeSchema) -> Self {
        let mut items: Vec<PageType> = Vec::new();

        let sections = [
            (resume.basics.is_some(), PageType::Overview),
            (!resume.work.is_empty(), PageType::Work),
            (!resume.education.is_empty(), PageType::Education),
            (!resume.skills.is_empty(), PageType::Skills),
            (!resume.interests.is_empty(), PageType::Interests),
            (!resume.languages.is_empty(), PageType::Languages),
            (
                resume.basics.is_some() && resume.basics.as_ref().unwrap().image.is_some(),
                PageType::Portrait,
            ),
        ];

        items.extend(
            sections
                .iter()
                .filter(|(condition, _)| *condition)
                .map(|(_, page_type)| page_type),
        );

        let mut state = ListState::default();
        state.select_first();

        Self { items, state }
    }
}

impl Default for App {
    fn default() -> Self {
        let content = std::fs::read_to_string("resume.json").unwrap();
        let resume = serde_json::from_str::<ResumeSchema>(&content).unwrap();

        Self {
            should_exit: false,
            pages: Pages::from(&resume),
            resume: Resume::from(resume),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::draw(frame, self);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => {
                self.handle_key(key);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.pages.state.select_next();
    }

    fn select_previous(&mut self) {
        self.pages.state.select_previous();
    }
}
