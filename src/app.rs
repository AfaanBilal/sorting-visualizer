/**
 * Sorting Visualizer
 *
 * @author Afaan Bilal (https://afaan.dev)
 * @link   https://github.com/AfaanBilal/sorting-visualizer
 */
use std::error;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{BarChart, Block, Borders, Paragraph};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current step of the algorithm.
    pub step_insertion: usize,
    /// Data to be sorted.
    pub data_insertion: Vec<u64>,
    /// Current step of the algorithm.
    pub step_bubble: usize,
    /// Data to be sorted.
    pub data_bubble: Vec<u64>,
    /// Current step of the algorithm.
    pub step_selection: usize,
    /// Data to be sorted.
    pub data_selection: Vec<u64>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,

            step_insertion: 0,
            data_insertion: vec![8, 3, 7, 1, 6, 2, 5, 9, 4],

            step_bubble: 0,
            data_bubble: vec![8, 3, 7, 1, 6, 2, 5, 9, 4],

            step_selection: 0,
            data_selection: vec![8, 3, 7, 1, 6, 2, 5, 9, 4],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sort the data using insertion sort.
    pub fn sort_insertion(&mut self) {
        if self.step_insertion < self.data_insertion.len() {
            for i in 0..self.data_insertion.len() - self.step_insertion - 1 {
                if self.data_insertion[i] > self.data_insertion[i + 1] {
                    self.data_insertion.swap(i, i + 1);
                }
            }

            self.step_insertion += 1;
        }
    }

    /// Sort the data using bubble sort.
    pub fn sort_bubble(&mut self) {
        if self.step_bubble < self.data_bubble.len() {
            for i in 0..self.step_bubble {
                for j in 0..self.data_bubble.len() - 1 - i {
                    if self.data_bubble[j] > self.data_bubble[j + 1] {
                        self.data_bubble.swap(j, j + 1);
                    }
                }
            }

            self.step_bubble += 1;
        }
    }

    pub fn sort_selection(&mut self) {
        if self.step_selection < self.data_selection.len() {
            for left in 0..self.step_selection {
                let mut smallest = left;

                for right in (left + 1)..self.data_selection.len() {
                    if self.data_selection[right] < self.data_selection[smallest] {
                        smallest = right;
                    }
                }

                self.data_selection.swap(smallest, left);
            }

            self.step_selection += 1;
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.sort_insertion();
        self.sort_bubble();
        self.sort_selection();
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let main_column = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(frame.size());

        let main_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(main_column[1]);

        frame.render_widget(
            Paragraph::new("Author: Afaan Bilal (https://afaan.dev)")
                .block(
                    Block::default()
                        .title("Sorting Visualizer")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center),
            main_column[0],
        );

        self.render_sort("Insertion", frame, main_row[0]);
        self.render_sort("Bubble", frame, main_row[1]);
        self.render_sort("Selection", frame, main_row[2]);
    }

    /// Render sort chart
    pub fn render_sort<B: Backend>(&mut self, algo: &str, frame: &mut Frame<'_, B>, area: Rect) {
        let data = match algo {
            "Insertion" => &self.data_insertion,
            "Bubble" => &self.data_bubble,
            "Selection" => &self.data_selection,
            _ => &self.data_insertion,
        };

        let step = match algo {
            "Insertion" => self.step_insertion,
            "Bubble" => self.step_bubble,
            "Selection" => self.step_selection,
            _ => self.step_insertion,
        };

        let bar_data = data.iter().map(|x| ("", *x)).collect::<Vec<_>>();

        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .title(format!("{} sort (Steps: {})", algo, step))
                    .borders(Borders::ALL),
            )
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

        frame.render_widget(bar_chart, area)
    }
}
