use std::{fs::File, sync::Arc};

use ::image::DynamicImage;
use anyhow::Context;
use image::fetch_image;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ListState,
    DefaultTerminal, Frame,
};
use tracing_subscriber::prelude::*;
use tui_scrollview::ScrollViewState;

pub mod date;
pub mod image;
pub mod json;
pub mod ui;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() -> anyhow::Result<()> {
    init_logging();
    let resume = parse_resume()?;
    let mut terminal = ratatui::init();
    let app_result = App::new(resume).run(&mut terminal);
    ratatui::restore();
    app_result
}

fn parse_resume() -> anyhow::Result<ResumeSchema> {
    let content =
        std::fs::read_to_string("resume.json").context("failed to read resume file contents")?;
    Ok(serde_json::from_str::<ResumeSchema>(&content)
        .context("failed to parse resume to schema")?)
}

fn init_logging() {
    let file = File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));
    tracing_subscriber::registry().with(debug_log).init();
}

#[derive(Debug)]
pub struct App {
    should_exit: bool,
    resume: Resume,
    pages: Pages,
    scroll_view_state: ScrollViewState,
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

impl Resume {
    fn get_email_url(&self) -> Option<String> {
        Some(format!(
            "mailto:{}",
            self.data.basics.as_ref()?.email.as_ref()?
        ))
    }

    fn get_phone_url(&self) -> Option<String> {
        Some(format!(
            "tel:{}",
            self.data.basics.as_ref()?.phone.as_ref()?
        ))
    }

    fn get_profile_url(&self, network: &str) -> Option<String> {
        let profiles: &Vec<ResumeSchemaBasicsProfilesItem> =
            self.data.basics.as_ref()?.profiles.as_ref();
        let profile = profiles
            .iter()
            .find(|p| p.network.as_ref().map(|s| s.to_lowercase()).as_deref() == Some(network))?;
        profile.url.clone()
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

    fn shortcuts(&self) -> Vec<Shortcut> {
        match self {
            Self::Overview => vec![
                Shortcut::OpenEmail,
                Shortcut::OpenPhone,
                Shortcut::OpenGithub,
                Shortcut::OpenBluesky,
                Shortcut::OpenTwitter,
            ],
            Self::Work => vec![Shortcut::OpenRigr, Shortcut::OpenPassle],
            _ => vec![],
        }
    }
}

enum Shortcut {
    Quit,
    OpenEmail,
    OpenPhone,
    OpenGithub,
    OpenBluesky,
    OpenTwitter,
    OpenRigr,
    OpenPassle,
}

impl Shortcut {
    fn label(&self) -> Line {
        match self {
            Self::Quit => shortcut_line("quit", 0),
            Self::OpenEmail => shortcut_line("email", 0),
            Self::OpenPhone => shortcut_line("phone", 0),
            Self::OpenGithub => shortcut_line("github", 0),
            Self::OpenBluesky => shortcut_line("bluesky", 0),
            Self::OpenTwitter => shortcut_line("twitter", 0),
            Self::OpenRigr => shortcut_line("rigr.gg", 0),
            Self::OpenPassle => shortcut_line("passle", 0),
        }
    }

    fn key(&self) -> KeyCode {
        match self {
            Self::Quit => KeyCode::Char('q'),
            Self::OpenEmail => KeyCode::Char('e'),
            Self::OpenPhone => KeyCode::Char('p'),
            Self::OpenGithub => KeyCode::Char('g'),
            Self::OpenBluesky => KeyCode::Char('b'),
            Self::OpenTwitter => KeyCode::Char('t'),
            Self::OpenRigr => KeyCode::Char('r'),
            Self::OpenPassle => KeyCode::Char('p'),
        }
    }

    fn handle(&self, app: &mut App) -> std::io::Result<()> {
        match self {
            Self::OpenEmail => open_url(app.resume.get_email_url()),
            Self::OpenPhone => open_url(app.resume.get_phone_url()),
            Self::OpenGithub => open_url(app.resume.get_profile_url("github")),
            Self::OpenBluesky => open_url(app.resume.get_profile_url("bluesky")),
            Self::OpenTwitter => open_url(app.resume.get_profile_url("twitter")),
            Self::OpenRigr => open_url(Some("https://rigr.gg".to_string())),
            Self::OpenPassle => open_url(Some("https://home.passle.net".to_string())),
            _ => Ok(()),
        }
    }
}

fn open_url(url_option: Option<String>) -> std::io::Result<()> {
    match url_option {
        Some(url) => open::that(&url),
        None => Ok(()),
    }
}

fn shortcut_line(label: &str, shortcut_offset: usize) -> Line {
    Line::from(vec![
        Span::styled(&label[..shortcut_offset], Style::default().fg(Color::Gray)),
        Span::styled(
            &label[shortcut_offset..shortcut_offset + 1],
            Style::default().fg(Color::White).bold(),
        ),
        Span::styled(
            &label[shortcut_offset + 1..],
            Style::default().fg(Color::Gray),
        ),
    ])
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

impl App {
    pub fn new(resume: ResumeSchema) -> Self {
        Self {
            should_exit: false,
            pages: Pages::from(&resume),
            resume: Resume::from(resume),
            scroll_view_state: ScrollViewState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_exit {
            terminal
                .draw(|frame| self.draw(frame))
                .context("failed to draw frame to terminal backend")?;
            self.handle_events()
                .context("an error occurred while handling events")?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::draw(frame, self);
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read().context("failed to read key event")? {
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
            KeyCode::Char('j') | KeyCode::Down => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.scroll_view_state.scroll_down()
                } else {
                    self.select_next()
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.scroll_view_state.scroll_up()
                } else {
                    self.select_previous()
                }
            }
            _ => {
                if let Some(selected) = self.pages.state.selected() {
                    let current_page = self.pages.items[selected];
                    let available_shortcuts = current_page.shortcuts();
                    for shortcut in available_shortcuts {
                        if key.code == shortcut.key() {
                            shortcut.handle(self).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn select_next(&mut self) {
        self.pages.state.select_next();
    }

    fn select_previous(&mut self) {
        self.pages.state.select_previous();
    }
}
