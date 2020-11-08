use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub enum CoreParams {
	/// Replace file name by specify 'str'.
	#[structopt(name = "replace")]
	Replace(ReplaceCmd),
	/// Insert string to file name.
	#[structopt(name = "insert")]
	Insert(InsertCmd),
	/// Do nothing.
	None,
}

/// params
#[derive(Clone, Debug, StructOpt)]
pub struct ReplaceCmd {
	#[structopt(flatten)]
	pub shared: SharedParam,
	/// To be replaced `src`
	#[structopt(long = "src")]
	pub src: String,
	/// Rename directory also.
	#[structopt(long = "rename-dir")]
	pub rename_dir: bool,
	/// Replaced by `dst`.
	#[structopt(long = "dst")]
	pub dst: Option<String>,
}

#[derive(Clone, Debug, StructOpt)]
pub struct SharedParam {
	/// Directory for to be renamed
	#[structopt(long = "dir", short = "d")]
	pub dir: String,
}

#[derive(Clone, Debug, StructOpt)]
pub struct InsertCmd {
	/// Shared params
	#[structopt(flatten)]
	pub shared: SharedParam,
	/// Position about which to be inserted.Can be `head`,`middle`,`tail`.
	#[structopt(long = "position", default_value = "middle")]
	pub position: String,
	#[structopt(long = "str")]
	pub str: String,
	#[structopt(long = "rename-dir")]
	pub rename_dir: bool,
}
