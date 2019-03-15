use clap::{self, App, Arg, ArgMatches};
use clap::{crate_authors, crate_version, crate_name, crate_description};


#[derive(Debug)]
pub enum Output {
  FileName,
  Bytes,
  Offset
}

impl Default for Output {
  fn default() -> Output { Output::FileName }
}


#[derive(Default, Debug)]
pub struct Options {
  pub inverse: bool,
  pub case_insensitive: bool,
  pub output: Output
}


#[derive(Default, Debug)]
pub struct Args {
  pub options: Options,
  pub pattern: String,
  pub files: Box<[String]>
}


#[derive(Debug)]
pub enum Command {
  Help(String),
  Version(String),
  Grep(Args)
}


#[derive(Debug)]
pub struct Error {
  pub message: String
}



fn build_app() -> App<'static, 'static> {
  App::new(crate_name!())
    .about(crate_description!())
    .author(crate_authors!())
    .version(crate_version!())
    // Positional arguments:
    .arg(
      Arg::with_name("pattern")
          .required(true)
          .index(1)
    )
    .arg(
      Arg::with_name("files")
        .multiple(true)
        .index(2)
    )
    // Matching flags:
    .arg(
      Arg::with_name("invert-match")
        .short("v")
        .long("invert-match")
        .help("inverse matching")
    )
    .arg(
      Arg::with_name("ignore-case")
        .short("i")
        .long("ignore-case")
        .help("case insensitive matching")
    )
    // Output flags:
    .arg(
      Arg::with_name("only-matching")
        .short("o")
        .long("only-matching")
        .help("print the matched bytes of each match")
        .overrides_with_all(&[
          "byte-offset",
          "files-with-matches",
          "files-without-matches",
        ])
    )
    .arg(
      Arg::with_name("byte-offset")
        .short("b")
        .long("byte-offset")
        .help("print the byte offset of each match")
        .overrides_with_all(&[
          "only-matching",
          "files-with-matches",
          "files-without-matches",
        ])
    )
    .arg(
      Arg::with_name("files-with-matches")
        .short("l")
        .long("files-with-matches")
        .help("print the name of the matched files")
        .overrides_with_all(&[
          "only-matching",
          "byte-offset",
          "files-without-matches",
        ])
    )
    .arg(
      Arg::with_name("files-without-matches")
        .short("L")
        .long("files-without-matches")
        .help("print the name of non-matched files (equivalent to `-vl`)")
        .overrides_with_all(&[
          "only-matching",
          "byte-offset",
          "files-with-matches",
        ])
    )
}


fn build_args<'a>(args: ArgMatches<'a>) -> Args {
  let pattern = String::from(
    args.value_of("pattern")
        .expect("<pattern> not in ArgMatches") // pattern is required.
  );

  let files = match args.values_of("files") {
    None     => Box::new([String::from("-")]) as Box<[String]>, // Input from stdin.
    Some(fs) => fs.map(String::from).collect()
  };

  let flag = |f| args.is_present(f);

  let output_flags = (
    flag("only-matching"),
    flag("byte-offset"),
    flag("files-with-matches"),
    flag("files-without-matches")
  );

  let output = match output_flags {
    (true, _, _, _) => Output::Bytes,
    (_, true, _, _) => Output::Offset,
    (_, _, true, _) => Output::FileName,
    (_, _, _, true) => Output::FileName,
    (_, _, _, _)    => Default::default(),
  };

  Args {
    options: Options {
      inverse: flag("invert-match") ^ flag("files-without-matches"), // (-L) is (-vl).
      case_insensitive: args.is_present("ignore-case"),
      output
    },
    pattern,
    files
  }
}


pub fn parse() -> Result<Command, Error> {
  let app = build_app();

  match app.get_matches_safe() {
    Ok(arg_matches) => Ok(Command::Grep(build_args(arg_matches))),
    Err(e) => match e.kind {
      clap::ErrorKind::HelpDisplayed    => Ok(Command::Help(e.message)),
      clap::ErrorKind::VersionDisplayed => Ok(Command::Version(e.message)),
      _ => Err(Error { message: e.message })
    }
  }
}
