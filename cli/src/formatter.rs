use colored::Colorize;
use tabled::settings::Style;
use tabled::{Table, Tabled};

pub fn default(string: String) {
    println!("{}", string);
}

pub fn default_table<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    println!("{}", print_table(iter));
}

pub fn success(string: String) {
    println!("\n{}: {}", "success".green(), string);
}

pub fn info(string: String) {
    println!("\n{}: {}", "info".blue(), string);
}

pub fn success_highlight(highlight: String, string: String) {
    println!("{} {}", highlight.green(), string);
}

pub fn success_table<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    println!("\n{}", print_table(iter).green());
}

pub fn error(string: String, path: &String) -> String {
    format!("\n{}: {} {}", "error".red(), string, print_path(path))
}

pub fn error_without_trace(string: String)  {
    println!("\n{}: {}", "error".red(), string)
}

pub fn error_highlight(highlight: String, string: String) {
    println!("{} {}", highlight.red(), string);
}

pub fn error_table<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    println!("{}", print_table(iter).red());
}

pub fn warning(string: String, path: &String) -> String {
    format!("\n{}: {} {}", "warning".yellow(), string, print_path(&path))
}

pub fn warning_highlight(highlight: String, string: String) {
    println!("{} {}", highlight.yellow(), string);
}

pub fn warning_table<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    println!("{}", print_table(iter).yellow());
}

fn print_table<I, T>(iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    Table::new(iter).with(Style::rounded()).to_string()
}

fn print_path(path: &String) -> String {
    format!("\n --> {}", &path.underline()).blue().to_string()
}
