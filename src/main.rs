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
		style::{Modifier, Style},
		widgets::{Block, BorderType, List, ListItem},
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
// struct State {
// inventory: Vec<(u8, String)>,
// }
#[derive(Default)]
enum Mode {
	#[default]
	Open,
	// Add,
	// Drop,
}
struct App {
	// state: State,
	mode: Mode,
}
impl App {
	fn new() -> Self {
		Self {
			// state: State { inventory: vec![] },
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
			Direction::Horizontal,
			[Constraint::Fill(1), Constraint::Fill(4)],
		)
		.split(frame.area());
		let left = Layout::new(
			Direction::Vertical,
			[Constraint::Fill(1), Constraint::Fill(4)],
		)
		.split(layout[0]);
		let inv_block = Block::bordered()
			.border_type(BorderType::Thick)
			.title("Inventory")
			.title_style(Style::new().add_modifier(Modifier::BOLD));
		frame.render_widget(&inv_block, left[0]);
		let mut inv_vec = vec![];
		for (c, s) in [
			('Q', "Add Items"),
			('W', "Drop Items"),
			('E', "Open Inventory"),
		] {
			inv_vec.push(ListItem::new(format!("[{c}] {s}")))
		}
		frame.render_widget(List::new(inv_vec), inv_block.clone().inner(left[0]));
		let menu_block = Block::bordered()
			.border_type(BorderType::Thick)
			.title("Menu")
			.title_style(Style::new().add_modifier(Modifier::BOLD));
		frame.render_widget(&menu_block, left[1]);
		let mut menu_vec = vec![];
		for (c, s) in [(1u8, "Sword"), (1u8, "Shield")] {
			menu_vec.push(ListItem::new(format!("{s}×{c}")))
		}
		frame.render_widget(List::new(menu_vec), menu_block.clone().inner(left[1]));
		frame.render_widget(
			Block::bordered()
				.border_type(BorderType::Thick)
				.title("Logs")
				.title_style(Style::new().add_modifier(Modifier::BOLD)),
			layout[1],
		);
	}
}
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
	let app = App::new();
	loop {
		app.update();
		terminal.draw(|frame| app.render(frame))?;
		if let Event::Key(key) = event::read()? {
			match key.code {
				KeyCode::Esc => break,
				KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {}
				KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {}
				KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {}
				KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {}
				_ => {}
			}
		}
	}
	Ok(())
}
