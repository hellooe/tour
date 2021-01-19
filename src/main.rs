use clap::{App, Arg, ArgMatches, SubCommand};
use tour::cmd;

fn main() {
    let app = App::new("tour")
        .about("做一些有趣的事")
        .subcommand(
            SubCommand::with_name("word")
                .about("将单词转化成特定格式")
                .arg(
                    Arg::with_name("case")
                        .help("请选择转换模式")
                        .takes_value(true)
                        .default_value("snake"),
                )
                .arg(
                    Arg::with_name("input")
                        .help("请输入单词内容")
                        .required(true)
                        .last(true),
                ),
        )
        .get_matches();

    match app.subcommand() {
        ("word", Some(m)) => word_cmd(m),
        _ => {}
    }
}

fn word_cmd(m: &ArgMatches) {
    let case = m.value_of("case").unwrap();
    let input = m.value_of("input").unwrap();
    let output = cmd::Words::new(input).into_case(case);
    println!("Output:\n{}", output);
}
