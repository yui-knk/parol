use miette::Result;

use parol::analysis::non_productive_non_terminals;
use parol::obtain_grammar_config;

pub fn sub_command() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("productivity")
        .about("Checks the given grammar for non-productive non-terminals.")
        .arg(
            clap::Arg::with_name("grammar_file")
                .required(true)
                .short("f")
                .long("grammar-file")
                .takes_value(true)
                .help("The grammar file to use")
        )
}

pub fn main(args: &clap::ArgMatches) -> Result<()> {
    let file_name = args
        .value_of("grammar_file")
        .unwrap();

    let grammar_config = obtain_grammar_config(&file_name, false)?;

    let non_productive_non_terminals = non_productive_non_terminals(&grammar_config.cfg);
    if non_productive_non_terminals.is_empty() {
        println!("No non-productive non-terminals found!");
    } else {
        println!("Non-productive non-terminals:");
        for nt in non_productive_non_terminals {
            println!("  {}", nt);
        }
    }
    Ok(())
}
