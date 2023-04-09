use std::error;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{BarChart, Block, BorderType, Borders, Paragraph};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current step of the algorithm.
    pub step: usize,
    /// Data to be sorted.
    pub data: Vec<u64>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            step: 0,
            data: vec![4, 6, 9, 10, 0, 1, 15, 26],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sort_step(&mut self) {
        if self.step < self.data.len() {
            for i in 0..self.data.len() - self.step - 1 {
                if self.data[i] > self.data[i + 1] {
                    self.data.swap(i, i + 1);
                }
            }
            self.step += 1;
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.sort_step();
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        frame.render_widget(
            Paragraph::new("Sorting Visualizer\nAfaan Bilal")
                .block(
                    Block::default()
                        .title("Template")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .alignment(Alignment::Center),
            frame.size(),
        );

        let bar_data = self.data.iter().map(|x| ("", *x)).collect::<Vec<_>>();

        let bar_chart = BarChart::default()
            .block(Block::default().borders(Borders::ALL))
            .data(&bar_data)
            .bar_width(7)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Green))
            .label_style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            )
            .value_style(Style::default().fg(Color::Black).bg(Color::Green));

        frame.render_widget(bar_chart, frame.size())
    }
}
