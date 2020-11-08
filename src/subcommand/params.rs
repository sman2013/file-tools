use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub enum CoreParams {
	/// Replace file name by specify 'str'.
	#[structopt(name = "replace")]
	Replace(ReplaceCmd),
	/// Insert string to file name.
	#[structopt(name = "insert")]
	Insert(InsertCmd),
	/// Rename file name by specify rules.
	#[structopt(name = "rename")]
	Rename(RenameCmd),
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
	/// Rename directory also.
	#[structopt(long = "rename-dir")]
	pub rename_dir: bool,
}

#[derive(Clone, Debug, StructOpt)]
pub struct RenameCmd {
	/// Shared params
	#[structopt(flatten)]
	pub shared: SharedParam,
	#[structopt(long = "shard-str", default_value = "")]
	pub shard_str: String,
	#[structopt(long = "connector", default_value = "")]
	pub connector: String,
	#[structopt(long = "sequence_mode", default_value = 0)]
	pub sequence_mode: int,
	#[structopt(long = "sequence_start", default_value = 0)]
	pub sequence_start: int,
	#[structopt(long = "sequence_position", default_value = 0)]
	pub sequence_position: int,
	#[structopt(long = "order_mode", default_value = 0)]
	pub order_mode: int,
}