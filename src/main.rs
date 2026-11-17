mod api;
mod ascii;
mod cache;
mod cli;
mod country;
mod display;
mod fmt;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use colored::Colorize;

use country::Country;

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

    let lookup = args.name.is_some()
        || args.capital.is_some()
        || args.random
        || args.list;
    let name_arg = args.name;
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

    if args.list {
        for country in &countries {
            println!("{}", country.name.common);
        }
        return Ok(());
    }

    if let Some(capital) = &args.capital {
        let country = country::find_by_capital(&countries, capital)?;
        if !emit_raw(args.json, args.field.as_deref(), country, &countries)? {
            display::print(country, &countries);
        }
    }

    let name = if args.random {
        Some(country::random(&countries)?.name.common.clone())
    } else {
        name_arg
    };

    if let Some(name) = &name {
        let country = country::find_by_name(&countries, name)?;
        if emit_raw(args.json, args.field.as_deref(), country, &countries)? {
            return Ok(());
        }

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

/// Handle `--json`/`--field`: prints the raw representation and reports
/// whether the normal display should be skipped.
fn emit_raw(
    json: bool,
    field: Option<&str>,
    country: &Country,
    all: &[Country],
) -> Result<bool> {
    if json {
        println!("{}", serde_json::to_string_pretty(country)?);
        return Ok(true);
    }
    if let Some(field) = field {
        match country.field_value(field, all)? {
            Some(value) => println!("{value}"),
            None => println!("-"),
        }
        return Ok(true);
    }
    Ok(false)
}
