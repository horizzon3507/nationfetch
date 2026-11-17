mod api;
mod ascii;
mod cache;
mod display;
mod cli;
mod country;
mod fmt;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use colored::Colorize;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err.to_string().red());
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args = cli::Cli::parse();
    let size = ascii::FlagSize {
        width: args.width,
        height: args.height,
    };
    let agent = api::agent();
    let dir = cache::dir()?;

    if args.sync {
        println!("Fetching countries data...");
        let raw = api::fetch_dataset(&agent)?;
        println!("Saving cache...");
        cache::save_dataset(&raw, &dir)?;
        println!("{}", "Cache saved.".green());

        if args.flag {
            println!(
                "Generating ASCII art for each country flag. This may take a minute..."
            );
            cache::cache_flags(&cache::read_countries(&dir)?, &dir, &size)?;
            println!("{}", "Done.".green());
        }
    }

    let lookup = args.name.is_some() || args.capital.is_some() || args.random;
    if !lookup {
        if args.flag_only {
            anyhow::bail!("--flagonly requires a country name");
        }
        if !args.sync {
            cli::Cli::command().print_help()?;
            println!();
        }
        return Ok(());
    }

    let countries = cache::read_countries(&dir)?;

    if let Some(capital) = &args.capital {
        display::print(country::find_by_capital(&countries, capital)?, &countries);
    }

    let name = if args.random {
        Some(country::random(&countries)?.name.common.clone())
    } else {
        args.name
    };

    if let Some(name) = &name {
        let country = country::find_by_name(&countries, name)?;
        let art = if args.flag || args.flag_only || args.flag_remote {
            if args.flag_remote {
                let png = api::fetch_flag_png(&agent, country)?;
                ascii::png_to_ascii(&png, &size)?
            } else {
                cache::read_flag(country, &dir)?
            }
        } else {
            String::new()
        };
        if args.flag_only {
            print!("{art}");
        } else if art.is_empty() {
            display::print(country, &countries);
        } else {
            display::print_side_by_side(&art, country, &countries);
        }
    }

    Ok(())
}
