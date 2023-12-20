extern crate clap;
use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};

pub fn get_socket_address() -> String {
    let args = get_arguments();
    let bind = args
        .get_one::<String>("bind")
        .expect("Failed to get the host address to bind the service")
        .clone();
    let port = args
        .get_one::<String>("port")
        .expect("Failed to get the port to make the service to listen to.")
        .clone();
    format!("{}:{}", bind, port)
}

pub fn get_bench_and_payload() -> (Option<usize>, Option<String>) {
    let args = get_arguments();
    let times = args.get_one("bench").copied();
    let payload = args.get_one("payload").cloned();
    (times, payload)
}

fn get_arguments<'a>() -> ArgMatches {
    Command::new("Memoizer")
        .version("0.1.2")
        .author("Sebastian Sastre <sebastianconcept@gmail.com>")
        .about("Minimalist thread-safe key-value store shared over TCP sockets.")
        .arg(
            Arg::with_name("bind")
                .value_name("IP_ADDRESS")
                .short('b')
                .long("bind")
                .multiple(false)
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .help("Defines an IP address to bind for listening incoming TCP connections.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .value_name("PORT")
                .short('p')
                .long("port")
                .multiple(false)
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .help("Defines the port to use.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bench")
                .value_name("NUM_TIMES")
                .short('e')
                .long("bench")
                .multiple(false)
                .action(ArgAction::Append)
                .value_parser(value_parser!(usize))
                .help("Runs a benchmark a number of times with an optional custom payload")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("payload")
                .value_name("CUSTOM_PAYLOAD")
                .short('d')
                .long("payload")
                .multiple(false)
                .action(ArgAction::Append)
                .value_parser(value_parser!(String))
                .help("The custom payload to use in a benchmark")
                .required(false)
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("bench_options")
                .args(&["bench", "payload"])
                .multiple(true)
                .required(false), // Either times or string is required
        )
        .group(
            ArgGroup::with_name("connection_options")
                .args(&["bind", "port"])
                .multiple(true)
                .required(false), // bind and port are required unless bench is selected
        )
        .get_matches()
}
