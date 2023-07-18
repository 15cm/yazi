use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}, widgets::{Block, BorderType, Borders, Clear, List, ListItem, Widget}};

use super::Ctx;

pub struct Select<'a> {
	cx: &'a Ctx,
}

impl<'a> Select<'a> {
	pub fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl<'a> Widget for Select<'a> {
	fn render(self, _: Rect, buf: &mut Buffer) {
		let select = &self.cx.select;
		let area = select.area();

		let items = select
			.window()
			.iter()
			.enumerate()
			.map(|(i, v)| {
				if i != select.rel_cursor() {
					return ListItem::new(format!("  {}", v));
				}

				ListItem::new(format!(" {}", v)).style(Style::default().fg(Color::Magenta))
			})
			.collect::<Vec<_>>();

		Clear.render(area, buf);
		List::new(items)
			.block(
				Block::default()
					.title(select.title())
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.border_style(Style::default().fg(Color::Blue)),
			)
			.render(area, buf);
	}
}
