use clap::{load_yaml, value_t_or_exit, App, ArgMatches};
use tour::cmd;

fn main() {
    let yaml = load_yaml!("tour.yaml");
    let app = App::from_yaml(yaml).get_matches();

    match app.subcommand() {
        ("word", Some(m)) => word_cmd(m),
        ("time", Some(m)) => time_cmd(m),
        _ => {}
    }
}

fn word_cmd(m: &ArgMatches) {
    let case = m.value_of("case").unwrap();
    let input = m.value_of("input").unwrap();
    let output = cmd::Words::new(input).into_case(case);
    println!("Output:\n{}", output);
}

fn time_cmd(m: &ArgMatches) {
    if m.is_present("weeks") {
        let weeks = value_t_or_exit!(m.value_of("weeks"), i64);
        let output = cmd::with_weeks(weeks);
        println!("Output:\n{}", output);
    }
    if m.is_present("timezone") {
        let timezone = m.value_of("timezone").unwrap();
        let output = cmd::with_timezone(timezone);
        println!("Output:\n{}", output);
    }
}
