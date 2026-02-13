use {
	crossterm::{
		event::{self, Event, KeyCode},
		execute,
		terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
	},
	ratatui::{
		Frame, Terminal,
		backend::CrosstermBackend,
		layout::{Constraint, Direction, Layout},
		style::{Color, Modifier, Style},
		widgets::{Block, BorderType, List, ListItem, Row, Table},
	},
	std::{
		default::Default,
		error::Error,
		io::{self, Stdout},
	},
};
fn main() -> Result<(), Box<dyn Error>> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen)?;
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
	let res = run(&mut terminal);
	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
	terminal.show_cursor()?;
	res
}
#[derive(Default)]
enum View {
	#[default]
	Main,
	Inv,
	PickUp,
	Drop,
}
struct State {
	view: View,
	nearby_items: Vec<(u8, String)>, // inventory: Vec<(u8, String)>,
}
#[derive(Default)]
enum Mode {
	#[default]
	Open,
	// Add,
	// Drop,
}
fn block<'a>() -> Block<'a> {
	Block::bordered().border_type(BorderType::Thick)
}
fn style() -> Style {
	Style::new()
}
struct App {
	state: State,
	mode: Mode,
}
impl App {
	fn new() -> Self {
		Self {
			state: State {
				view: View::default(),
				nearby_items: vec![(1u8, "Sword".to_string()), (1u8, "Shield".to_string())],
			},
			mode: Mode::default(),
		}
	}
	fn update(&self) {
		match self.mode {
			Mode::Open => {} // Mode::Add => {}
			                 // Mode::Drop => {}
		}
	}
	fn render(&self, frame: &mut Frame) {
		let layout = Layout::new(
			Direction::Vertical,
			[
				Constraint::Fill(1),
				Constraint::Fill(3),
				Constraint::Fill(1),
			],
		)
		.split(frame.area());
		let con = layout[0];
		let con_block = block()
			.title("Controls")
			.title_style(Style::new().add_modifier(Modifier::BOLD));
		frame.render_widget(&con_block, con);
		let mut con_vec = vec![];
		for (c, s) in [
			(KeyCode::Esc, "Quit Game"),
			(KeyCode::Char('q'), "Return to Main"),
			(KeyCode::Char('w'), "Pick-up Item(s)"),
			(KeyCode::Char('e'), "Drop Item(s)"),
			(KeyCode::Char('r'), "Open Inventory"),
		] {
			con_vec.push(format!("[{}] {s}", c.to_string().to_uppercase()))
		}
		let mut widths = vec![];
		for _ in 0..3 {
			widths.push(Constraint::Fill(1));
		}
		let mut rows = vec![];
		let mut row = vec![];
		for (i, n) in con_vec.clone().into_iter().enumerate() {
			row.push(n);
			if (i + 1) % 3 == 0 {
				rows.push(Row::new(row));
				row = vec![];
			}
		}
		rows.push(Row::new(row));
		frame.render_widget(Table::new(rows, widths), con_block.clone().inner(con));
		let middle = Layout::new(
			Direction::Horizontal,
			[Constraint::Fill(3), Constraint::Fill(2)],
		)
		.split(layout[1]);
		let view_block = block()
			.title("View")
			.title_style(Style::new().add_modifier(Modifier::BOLD));
		frame.render_widget(&view_block, middle[0]);
		match self.state.view {
			View::Main => {}
			View::Inv => {
				let mut view_vec = vec![];
				for (c, s) in &self.state.nearby_items {
					view_vec.push(ListItem::new(format!("{s}×{c}")))
				}
				frame.render_widget(List::new(view_vec), view_block.clone().inner(middle[0]));
			}
			View::PickUp => {}
			View::Drop => {}
		}
		frame.render_widget(
			block()
				.title("Logs")
				.title_style(Style::new().add_modifier(Modifier::BOLD)),
			middle[1],
		);
		let stats = Layout::new(
			Direction::Horizontal,
			[
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
				Constraint::Fill(1),
			],
		)
		.split(layout[2]);
		frame.render_widget(
			block().title("HP").border_style(style().fg(Color::Green)),
			stats[0],
		);
		frame.render_widget(
			block()
				.title("Mana")
				.border_style(style().fg(Color::Magenta)),
			stats[1],
		);
		frame.render_widget(
			block().title("Atk").border_style(style().fg(Color::Red)),
			stats[2],
		);
		frame.render_widget(
			block().title("Def").border_style(style().fg(Color::Blue)),
			stats[3],
		);
		frame.render_widget(
			block()
				.title("Gold")
				.border_style(style().fg(Color::Yellow)),
			stats[4],
		);
	}
}
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
	let mut app = App::new();
	loop {
		app.update();
		terminal.draw(|frame| app.render(frame))?;
		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Esc => break,
				KeyCode::Char('q') => app.state.view = View::Main,
				KeyCode::Char('w') => app.state.view = View::PickUp,
				KeyCode::Char('e') => app.state.view = View::Drop,
				KeyCode::Char('r') => app.state.view = View::Inv,
				_ => {}
			}
		}
	}
	Ok(())
}
