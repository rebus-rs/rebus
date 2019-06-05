use std::env;
use structopt::{StructOpt, clap::{App, AppSettings, Arg, SubCommand}};
use lazy_static::lazy_static;
use rebus::Settings;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new()
        .expect("Read settings error");
}

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(about = "The Red Event Bus utility.", author = "")]
enum Rebus {
    #[structopt(name = "trigger", about = "Triggering events by external start point.", author = "")]
    #[structopt(raw(settings = "&[AppSettings::TrailingVarArg, AppSettings::AllowLeadingHyphen]"))]
    Trigger {
        args: Vec<String>,
    },
}

fn rebus_app<'a, 'b>(external_subcommands: &[&'a str]) -> App<'a, 'b> {
    let mut clap = Rebus::clap();
    for &name in external_subcommands {
        clap = clap
            .subcommand(SubCommand::with_name(name)
                .settings(&[
                    AppSettings::TrailingVarArg,
                    AppSettings::AllowLeadingHyphen,
                    AppSettings::DisableHelpFlags,
                    AppSettings::DisableVersion])
                .arg(Arg::with_name("args")
                    .multiple(true)));
    }
    clap
}

fn main() {
    let external_subcommands = SETTINGS.extensions.iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();
    let matches = rebus_app(&external_subcommands)
        .get_matches();

    if let Some(name) = matches.subcommand_name()
        .and_then(|name| external_subcommands.iter()
            .find(|&&item| item == name))
    {
        println!("External subcommand: {:?}", name);
        for arg in env::args().skip(1) {
            println!("{:?}", arg);
        }
    } else {
        let rebus = Rebus::from_clap(&matches);
        println!("{:?}", rebus);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_trigger_subcommand() {
        let samples = [
            vec!["rebus", "trigger", "git", "status"],
            vec!["rebus", "trigger", "git", "commit", "-am", "\"Message text\""],
            vec!["rebus", "trigger", "-foo", "bar", "--", "-a"],
        ];

        for sample in &samples {
            let matches = rebus_app(&[])
                .get_matches_from(sample);
            let args = sample[2..].iter().map(|s| s.to_string()).collect();

            assert_eq!(Rebus::Trigger { args }, Rebus::from_clap(&matches));
        }
    }

    #[test]
    fn matches_external_subcommands() {
        let samples = [
            vec!["rebus", "git", "status"],
            vec!["rebus", "git", "commit", "-am", "\"Message text\""],
            vec!["rebus", "foo", "-m", "bar", "--", "-a"],
            vec!["rebus", "foo", "-h", "--help", "-V", "--version"],
            vec!["rebus", "foo", "--help", "-V", "--version", "-h"],
            vec!["rebus", "foo", "-V", "--version", "-h", "--help"],
            vec!["rebus", "foo", "--version", "-h", "--help", "-V"],
        ];

        for sample in &samples {
            let command = sample[1];
            let matches = rebus_app(&[command])
                .get_matches_from(sample);

            assert_eq!(matches.subcommand_name(), Some(command));
            assert_eq!(
                matches.subcommand_matches(command).unwrap()
                    .values_of("args").unwrap()
                    .collect::<Vec<_>>(),
                &sample[2..]
            );
        }
    }
}