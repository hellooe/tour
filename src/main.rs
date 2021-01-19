use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use tour::cmd;

fn main() {
    let app = App::new("tour")
        .about("做一些有趣的事")
        .subcommand(
            SubCommand::with_name("word")
                .about("转化格式")
                .arg(
                    Arg::with_name("case")
                        .short("c")
                        .long("case")
                        .help("请选择转换模式")
                        .takes_value(true)
                        .default_value("snake"),
                )
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("请输入单词内容")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("time")
                .about("计算时间")
                .arg(
                    Arg::with_name("weeks")
                        .short("w")
                        .long("weeks")
                        .help("输入周数")
                        .takes_value(true)
                        .conflicts_with("timezone"),
                )
                .arg(
                    Arg::with_name("timezone")
                        .short("t")
                        .long("timezone")
                        .help("输入时区")
                        .takes_value(true),
                ),
        )
        .get_matches();

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
