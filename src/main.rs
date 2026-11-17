mod api;
mod cache;
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

    if args.sync {
        let agent = api::agent();
        println!("Fetching countries data...");
        let countries = api::fetch_countries(&agent)?;
        println!("Saving cache...");
        cache::save_countries(&countries, &cache::dir()?)?;
        println!("{}", "Cache saved.".green());
    }

    let lookup = args.name.is_some() || args.capital.is_some() || args.random;
    if !lookup {
        if !args.sync {
            cli::Cli::command().print_help()?;
            println!();
        }
        return Ok(());
    }

    let countries = cache::read_countries(&cache::dir()?)?;

    if let Some(capital) = &args.capital {
        country::find_by_capital(&countries, capital)?.print();
    }

    let name = if args.random {
        Some(country::random(&countries)?.name.common.clone())
    } else {
        args.name
    };

    if let Some(name) = &name {
        let country = country::find_by_name(&countries, name)?;
        println!();
        country.print();
    }

    Ok(())
}
