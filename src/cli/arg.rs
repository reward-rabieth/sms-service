use clap::{Command, Arg};

pub fn build_cli() -> Command {

    Command::new("sms_service")
        .about("speaker management CLI")
        .subcommand(Command::new("add-speaker").
            about("Add a new speaker").arg(Arg::new("name")
            .short('n')
            .long("name")
            .required(true).help("Full name of the speaker"),
        )
            .arg(Arg::new("phone")
                .short('p')
                .long("phone")
                .required(true).help("Phone number of the speaker"),
            )
            .arg(Arg::new("date")
                .short('d')
                .long("date")
                .required(false).help("Assignment date in RFC3339 format (ISO 8601). Defaults to now."),)
        )
}
