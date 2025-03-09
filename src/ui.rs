use anyhow::Context;
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect, Size},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, List, Padding, Paragraph, Widget, Wrap},
    Frame,
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{App, PageType, Shortcut};

const SIDEBAR_PADDING_LEFT: usize = 2;
const CONTENT_PADDING_LEFT: usize = 3;
const PADDING_VERTICAL: u16 = 1;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].into_iter())
        .split(frame.area());

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Min(20)].into_iter())
        .split(outer[0]);

    draw_sidebar(frame, layout[0], app).unwrap();
    draw_content(frame, layout[1], app).unwrap();

    draw_status_bar(frame, outer[1], app).unwrap();
}

fn draw_sidebar(frame: &mut Frame, area: Rect, app: &mut App) -> anyhow::Result<()> {
    let page_labels = app
        .pages
        .items
        .iter()
        .map(|page_type| format!("{}{}", " ".repeat(SIDEBAR_PADDING_LEFT), page_type.label()));
    let list = List::new(page_labels)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(format!("{}Pages", "─".repeat(SIDEBAR_PADDING_LEFT)))
                .title_bottom(" j/↓ k/↑ "),
        )
        .style(Style::new().white())
        .highlight_style(Style::new().bold().bg(Color::DarkGray));
    frame.render_stateful_widget(list, area, &mut app.pages.state);
    Ok(())
}

fn draw_content(frame: &mut Frame, area: Rect, app: &mut App) -> anyhow::Result<()> {
    let content = Block::bordered()
        .border_type(BorderType::Rounded)
        .padding(ratatui::widgets::Padding {
            left: 3,
            right: 3,
            top: PADDING_VERTICAL,
            bottom: PADDING_VERTICAL,
        })
        .title(format!("{}Content", "─".repeat(CONTENT_PADDING_LEFT)))
        .title_bottom(" alt+j/↓, alt+k/↑ ");

    let selected = match app.pages.state.selected() {
        Some(selected) => selected,
        None => return Ok(()),
    };
    match app.pages.items[selected] {
        PageType::Overview => {
            draw_overview(frame, content.inner(area), app).context("failed to draw overview")?
        }
        PageType::Portrait => draw_portrait(frame, content.inner(area), app)
            .context("failed to draw ascii portrait")?,
        _ => {}
    };

    frame.render_widget(content, area);

    Ok(())
}

fn draw_status_bar(frame: &mut Frame, area: Rect, app: &mut App) -> anyhow::Result<()> {
    let bar = Block::default()
        .style(Style::default().bg(Color::DarkGray))
        .padding(ratatui::widgets::Padding {
            left: 2,
            right: 2,
            top: 0,
            bottom: 0,
        });

    draw_status_bar_shortcuts(
        frame,
        bar.inner(area),
        StatusBarSide::Left,
        vec![Shortcut::Quit],
    )
    .context("draw global shortcuts")?;

    let content_shortcuts = match app.pages.state.selected() {
        Some(selected) => app.pages.items[selected].shortcuts(),
        None => vec![],
    };

    if !content_shortcuts.is_empty() {
        draw_status_bar_shortcuts(
            frame,
            bar.inner(area),
            StatusBarSide::Right,
            content_shortcuts,
        )
        .context("draw content shortctus")?;
    }

    frame.render_widget(bar, area);

    Ok(())
}

enum StatusBarSide {
    Left,
    Right,
}

fn draw_status_bar_shortcuts(
    frame: &mut Frame,
    area: Rect,
    side: StatusBarSide,
    shortcuts: Vec<Shortcut>,
) -> anyhow::Result<()> {
    let layout = Layout::horizontal(
        shortcuts
            .iter()
            .map(|s| Constraint::Length(u16::try_from(s.label().width()).unwrap())),
    )
    .flex(match side {
        StatusBarSide::Left => Flex::Start,
        StatusBarSide::Right => Flex::End,
    })
    .spacing(2);

    let areas = layout.split(area);
    let areas = areas.iter().collect_vec();
    for (shortcut, rect) in shortcuts.iter().zip(areas) {
        frame.render_widget(shortcut.label(), *rect);
    }

    Ok(())
}

fn draw_overview(frame: &mut Frame, area: Rect, app: &mut App) -> anyhow::Result<()> {
    let basics = app.resume.data.basics.as_ref().unwrap();
    let mut lines = Vec::new();

    if let Some(name) = basics.name.as_ref() {
        lines.push(Line::styled(name, Style::default().bold()));
        lines.push(Line::default());
    }

    push_if_some(&mut lines, &basics.summary, "", true)?;

    if let Some(location) = basics.location.as_ref() {
        let city = location.city.as_deref().unwrap_or("");
        let country = location.country_code.as_deref().unwrap_or("");

        let location_str = match (city.is_empty(), country.is_empty()) {
            (false, false) => format!("{}, {}", city, country),
            (false, true) => city.to_string(),
            (true, false) => country.to_string(),
            (true, true) => String::new(),
        };

        if !location_str.is_empty() {
            lines.push(Line::from(format!("📌 {}", location_str)));
            lines.push(Line::default());
        }
    }

    push_if_some(&mut lines, &basics.email, "✉️ ", false)?;
    push_if_some(&mut lines, &basics.phone, "☎️ ", true)?;

    basics.profiles.iter().for_each(|profile| {
        let network = profile.network.as_deref().unwrap_or("");
        let icon = match network.to_lowercase().as_str() {
            "github" => "😸",
            "bluesky" => "🦋",
            "twitter" => "🐦",
            _ => "🔗",
        };

        let username = profile.username.as_deref().unwrap_or("");
        let url = profile.url.as_deref().unwrap_or("");

        let profile_str = match (username.is_empty(), url.is_empty()) {
            (true, false) | (false, false) => url.to_string(),
            (false, true) => username.to_string(),
            (true, true) => String::new(),
        };

        if !profile_str.is_empty() {
            lines.push(Line::from(format!("{} {}", icon, profile_str)));
        }
    });

    draw_scrollview(frame, area, &mut app.scroll_view_state, lines)?;

    Ok(())
}

fn draw_scrollview(
    frame: &mut Frame,
    area: Rect,
    scroll_view_state: &mut ScrollViewState,
    lines: Vec<Line>,
) -> anyhow::Result<()> {
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::new().padding(Padding {
            left: 0,
            right: 2,
            top: 0,
            bottom: 0,
        }));

    let content_size = Size::new(area.width, u16::try_from(paragraph.line_count(area.width))?);
    let mut scroll_view =
        ScrollView::new(content_size).horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    scroll_view.render_widget(
        paragraph,
        Rect::new(0, 0, content_size.width, content_size.height),
    );

    frame.render_stateful_widget(scroll_view, area, scroll_view_state);

    Ok(())
}

fn draw_portrait(frame: &mut Frame, area: Rect, app: &mut App) -> anyhow::Result<()> {
    if let Some(image) = app.resume.cached_image.as_ref() {
        let widget = AsciiImage::new(image);
        frame.render_widget(widget, area);
    }

    Ok(())
}

fn push_if_some(
    lines: &mut Vec<Line>,
    opt: &Option<String>,
    prefix: &str,
    push_newline: bool,
) -> Result<(), anyhow::Error> {
    if let Some(value) = opt {
        lines.push(Line::from(format!("{}{}", prefix, value)));
        if push_newline {
            lines.push(Line::default());
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AsciiImage<'a> {
    image: &'a DynamicImage,
}

impl<'a> AsciiImage<'a> {
    pub fn new(image: &'a DynamicImage) -> Self {
        Self { image }
    }

    fn resize_image(&self, area: &Rect) -> DynamicImage {
        let (area_width, area_height) = (area.width as f32, area.height as f32);
        let (image_width, image_height) = (
            (self.image.width() as f32) * 2.0,
            self.image.height() as f32,
        );

        let area_ratio = area_width / area_height;
        let image_ratio = image_width / image_height;

        let (scaled_width, scaled_height) = if area_ratio > image_ratio {
            (image_width * area_height / image_height, area_height)
        } else {
            (area_width, image_height * area_width / image_width)
        };

        self.image.resize_exact(
            scaled_width as u32,
            scaled_height as u32,
            FilterType::Nearest,
        )
    }
}

const ASCII_CHARS: &str =
    "@&%QWNM0gB$#DR8mHXKAUbGOpV4d9h6PkqwSE2]ayjxY5Zoen[ult13If}C{iF|(7J)vTLs?z/*cr!+<>;=^,_:'-.` "; // Dark to light

impl<'a> Widget for AsciiImage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let resized = self.resize_image(&area);

        let top = area.top() + (area.height - resized.height() as u16) / 2;
        let left = area.left() + (area.width - resized.width() as u16) / 2;

        for (x, y, rgba) in resized.pixels() {
            let char = {
                let luma = rgba.to_luma().0[0];
                let index = (luma as usize * (ASCII_CHARS.len() - 1)) / 255;
                ASCII_CHARS.chars().nth(index).unwrap()
            };

            buf.set_string(
                left + x as u16,
                top + y as u16,
                format!("{char}"),
                Style::default().fg(Color::Rgb(rgba.0[0], rgba.0[1], rgba.0[2])),
            );
        }
    }
}
