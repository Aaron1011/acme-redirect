mod acme;
mod args;
mod cert;
mod chall;
mod config;
mod daemon;
mod errors;
mod http_responses;
mod persist;
mod renew;
mod status;

use crate::args::{Args, Cmd, SubCommand};
use crate::errors::*;
use env_logger::Env;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    let logging = match args.verbose {
        0 => "info",
        1 => "info,acme_redirect=debug",
        2 => "debug",
        _ => "debug,acme_redirect=trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(logging));

    match args.subcommand.clone() {
        SubCommand::Cmds(subcommand) => {
            let config = config::load(&args)?;
            trace!("Loaded runtime config: {:?}", config);

            match subcommand {
                Cmd::Daemon(args) => daemon::run(config, args)?,
                Cmd::Status => status::run(config)?,
                Cmd::Renew(args) => renew::run(config, args)?,
            }
        }
        SubCommand::Completions(completions) => args::gen_completions(&completions)?,
    }

    Ok(())
}
