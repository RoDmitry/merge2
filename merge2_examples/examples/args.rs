use clap::Parser;
use merge2::Merge;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Merge, Parser)]
#[serde(default)]
struct Args {
    #[arg(short, long)]
    #[merge(strategy = ::merge2::bool::overwrite_false)]
    debug: bool,
    input: Option<String>,
    output: Option<String>,
}

fn get_config() -> Option<Args> {
    let path: &std::path::Path = "args.toml".as_ref();
    if path.is_file() {
        let s = std::fs::read_to_string(path).expect("Could not read configuration file");
        Some(toml::from_str(&s).expect("Could not parse configuration"))
    } else {
        None
    }
}

fn get_env() -> Args {
    envy::prefixed("ARGS_")
        .from_env()
        .expect("Could not read environment variables")
}

fn main() {
    let mut args = Args::parse();
    args.merge(&mut get_env());
    if let Some(mut config) = get_config() {
        args.merge(&mut config);
    }
    println!("{:?}", args);
}
