use std::io;

mod app;
mod subcommand;

fn main() -> io::Result<()> {
	let args = std::env::args();

	match app::parse_and_execute(args) {
		Err(e) => {
			println!("{:?}", e);
		}
		_ => {}
	}

	Ok(())
}
