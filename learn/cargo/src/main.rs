use druid::widget::{Label, Container, Flex, Split};
use druid::{AppLauncher, Widget, WindowDesc};

mod file;

fn build_ui() -> impl Widget<()> {
	Split::columns(
		Container::new(
			Flex::column()
				.with_flex_child(Label::new("1. Kind"), 2.0)
				.with_flex_child(Label::new("2. Kind"), 1.0),
		),
		Container::new(
			Flex::row()
				.with_flex_child(Label::new("1. Kind"), 2.0)
				.with_flex_child(Label::new("2. Kind"), 1.0),
		),
	)
	.solid_bar(true)
	.draggable(true)
	.split_point(0.3)
	.bar_size(0.5)
}

fn main() {
	let main_window = WindowDesc::new(build_ui())
		.window_size((600.0, 400.0))
		.title("Kontakte");
	let initial_data = ();

	AppLauncher::with_window(main_window)
		.launch(initial_data)
		.expect("Failed to launch application");
}
