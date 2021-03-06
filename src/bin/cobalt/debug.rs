use clap;
use cobalt;

use args;
use error::*;

pub fn debug_command_args() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("debug")
        .about("Print site debug information")
        .subcommand(clap::SubCommand::with_name("config").about("Prints post-processed config"))
        .subcommand(
            clap::SubCommand::with_name("highlight")
                .about("Print syntax-highlight information")
                .subcommand(clap::SubCommand::with_name("themes"))
                .subcommand(clap::SubCommand::with_name("syntaxes")),
        ).subcommand(
            clap::SubCommand::with_name("files")
                .about("Print files associated with a collection")
                .args(&args::get_config_args())
                .arg(
                    clap::Arg::with_name("COLLECTION")
                        .help("Collection name")
                        .index(1),
                ),
        )
}

pub fn debug_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("config", _) => {
            let config = args::get_config(matches)?;
            let config = config.build()?;
            println!("{}", config);
        }
        ("highlight", Some(matches)) => match matches.subcommand() {
            ("themes", _) => for name in cobalt::list_syntax_themes() {
                println!("{}", name);
            },
            ("syntaxes", _) => for name in cobalt::list_syntaxes() {
                println!("{}", name);
            },
            _ => bail!(matches.usage()),
        },
        ("files", Some(matches)) => {
            let config = args::get_config(matches)?;
            let config = config.build()?;
            let collection = matches.value_of("COLLECTION");
            match collection {
                Some("assets") => {
                    let assets = config.assets.build()?;
                    for file_path in assets.files() {
                        println!("{:?}", file_path);
                    }
                }
                Some("pages") => {
                    let pages = config.pages.build()?;
                    for file_path in pages.pages.files() {
                        println!("{:?}", file_path);
                    }
                    if let Some(ref drafts) = pages.drafts {
                        for file_path in drafts.files() {
                            println!("{:?}", file_path);
                        }
                    }
                }
                Some("posts") => {
                    let posts = config.posts.build()?;
                    for file_path in posts.pages.files() {
                        println!("{:?}", file_path);
                    }
                    if let Some(ref drafts) = posts.drafts {
                        for file_path in drafts.files() {
                            println!("{:?}", file_path);
                        }
                    }
                }
                None => {
                    bail!("Must specify collection");
                }
                _ => {
                    bail!("Collection is not yet supported");
                }
            }
        }
        _ => bail!(matches.usage()),
    }

    Ok(())
}
