use chrono::{Datelike, Local, Month, NaiveDate};
use std::env;
use std::io::{self, Write}; // Added Write trait import
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Function to print a colored string
fn print_colored(text: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(stdout, "{}", text)?;
    stdout.reset()?;
    Ok(())
}

// Function to get the number of days in a given month and year
fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}

// Function to get the weekday of the first day of the month (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
fn first_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap()
        .weekday()
        .num_days_from_sunday()
}

// Function to print the calendar for a given year and month
fn print_calendar(year: i32, month: u32) -> io::Result<()> {
    let month_name = Month::try_from(month as u8).unwrap().name();
    let days = days_in_month(year, month);
    let first_day = first_day_of_month(year, month);

    // Print header with month and year
    print_colored(
        &format!("\n{:^44}\n", format!("{} {}", month_name, year)),
        Color::Cyan,
    )?;
    print_colored(
        "--------------------------------------------\n",
        Color::White,
    )?;
    print_colored(" Sun  Mon  Tue  Wed  Thu  Fri  Sat\n", Color::Yellow)?;
    print_colored(
        "--------------------------------------------\n",
        Color::White,
    )?;

    // Print leading spaces for the first week
    for _ in 0..first_day {
        print!("     ");
    }

    // Print days of the month
    let mut current_day = 1;
    let mut weekday = first_day;
    while current_day <= days {
        if weekday == 0 {
            print!("\n"); // Start a new week
        }
        print!(" {:>3} ", current_day);
        if weekday == 6 {
            print!("\n"); // End the week
        }
        current_day += 1;
        weekday = (weekday + 1) % 7;
    }

    // Ensure the calendar ends with a newline
    if weekday != 0 {
        print!("\n");
    }
    print_colored(
        "--------------------------------------------\n\n",
        Color::White,
    )?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (year, month) = if args.len() == 3 {
        (
            args[1].parse::<i32>().unwrap_or(Local::now().year()),
            args[2].parse::<u32>().unwrap_or(Local::now().month()),
        )
    } else {
        (Local::now().year(), Local::now().month())
    };

    // Validate month
    if month < 1 || month > 12 {
        eprintln!("Invalid month. Please enter a month between 1 and 12.");
        return Ok(());
    }

    print_calendar(year, month)?;
    Ok(())
}
