use std::path::{PathBuf};
use clap::{Parser};
use regex::{Regex};
use walkdir::{WalkDir};
use anyhow::{Context, Result, Ok, bail};
use ansi_term::{ANSIString};
use ansi_term::Style;
use ansi_term::Colour::{Cyan, Green};
use std::fs::create_dir_all;
use fs_extra;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(author, version,
about = "Bulk rename using regular expressions",
long_about = "A fast and simple tool for copying data from an input directory to an output directory under different paths based on regular expressions.\nThe syntax is based on the \"regex\" crate: see https://docs.rs/regex/1.5.5/regex/#grouping-and-flags",
propagate_version = true,
disable_help_subcommand = true
)]
struct Cli {
    /// Input path filter. Paths which do not match this regex are excluded.
    #[clap(short, long, default_value = ".*")]
    filter: String,

    /// Regular expression to match paths
    #[clap(short, long, default_value = "(.*)")]
    expression: String,

    /// Replacement string with capture groups
    #[clap(short, long, default_value = "$0")]
    replace: String,

    /// Silence output
    #[clap(short, long)]
    quiet: bool,

    /// deprecated ChRIS flag. Does nothing.
    #[clap(long)]
    saveinputmeta: bool,

    /// deprecated ChRIS flag. Does nothing.
    #[clap(long)]
    saveoutputmeta: bool,

    /// input directory
    #[clap()]
    input_dir: PathBuf,

    /// output directory
    #[clap()]
    output_dir: PathBuf,
}


fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    if !args.input_dir.is_dir() {
        bail!("not a directory: {:?}", args.input_dir);
    }
    if !args.output_dir.is_dir() {
        bail!("not a directory: {:?}", args.output_dir);
    }
    if !args.output_dir.read_dir()?.next().is_none() {
        bail!("not empty: {:?}", args.output_dir);
    }

    let filter = Regex::new(&args.filter)
        .with_context(|| format!("Invalid option --filter={}", &args.filter))?;
    let expression = Regex::new(&args.expression)
        .with_context(|| format!("Invalid option --expression={}", &args.expression))?;

    let input_pre = args.input_dir.to_str().unwrap();
    let output_pre = args.output_dir.to_str().unwrap();

    let mut did_nothing = true;

    for (rel, input_path) in filter_input_dir(&args.input_dir, &filter) {
        let renamed = expression.replace(rel.to_str().unwrap(), &args.replace).to_string();
        let output_path = args.output_dir.join(&renamed);

        if output_path.exists() {
            bail!(
                "{:?} already exists. Hint: to operate on subdirectories, try --filter='^{}$'",
                &output_path,
                args.filter
            );
        }

        cpr(&input_path, &output_path)?;
        pretty_print(input_pre, output_pre, &rel, &renamed);
        did_nothing = false;
    }

    if did_nothing {
        bail!("No paths under {:?} matched by --filter={}", &args.input_dir, &args.filter)
    }
    Ok(())
}

/// Pretty much `cp -r $1 $2`
fn cpr(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    let parent_dir = dst.parent().unwrap();
    create_dir_all(parent_dir)
        .with_context(|| format!("Could not create parent directory {:?}", parent_dir))?;
    if src.is_file() {
        fs_extra::file::copy(src, dst, &*FILE_COPY_OPTIONS)?;
    }
    else if src.is_dir() {
        fs_extra::dir::copy(src, dst, &*DIR_COPY_OPTIONS)?;
    }
    else {
        bail!("{:?} is not a file nor directory", src);
    }
    Ok(())
}


/// produce relative subpaths under a directory which match a regex
fn filter_input_dir<'a>(
    input_dir: &'a PathBuf, filter: &'a Regex
) -> impl Iterator<Item = (PathBuf, PathBuf)> +'a {
    WalkDir::new(input_dir).into_iter()
        .map(|e| e.unwrap().into_path())
        .map(move |p| ((p.strip_prefix(input_dir).unwrap()).to_owned(), p))
        .filter(|e| filter.is_match(e.0.to_string_lossy().as_ref()))
}

fn pretty_print(input_pre: &str, output_pre: &str, src: &PathBuf, dst: &str) {
    println!(
        "{}/{} {} {}/{}",
        input_pre,
        Cyan.paint(src.to_str().unwrap()),
        *DIM_ARROW,
        output_pre,
        Green.paint(dst)
    )
}

lazy_static! {
    static ref DIM_ARROW: ANSIString<'static> = Style::new().dimmed().paint("->");
    static ref FILE_COPY_OPTIONS: fs_extra::file::CopyOptions = fs_extra::file::CopyOptions::new();
    // static ref DIR_COPY_OPTIONS: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new();
    static ref DIR_COPY_OPTIONS: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions{
        overwrite: false,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        content_only: true,
        depth: 0
    };
}