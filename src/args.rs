use structopt::clap::AppSettings;
use structopt::StructOpt;

const LETSENCRYPT: &str = "https://acme-v02.api.letsencrypt.org/directory";
// const LETSENCRYPT_STAGING: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
pub struct Args {
    #[structopt(short, long, global = true)]
    pub verbose: bool,
    #[structopt(
        short,
        long,
        value_name = "path",
        default_value = "/etc/acme-redirect.conf",
        env = "ACME_CONFIG_DIR"
    )]
    pub config: String,
    #[structopt(
        long,
        value_name = "path",
        default_value = "/etc/acme-redirect.d",
        env = "ACME_CONFIG_DIR"
    )]
    pub config_dir: String,
    #[structopt(
        long,
        value_name = "path",
        default_value = "/run/acme-redirect",
        env = "ACME_CHALL_DIR"
    )]
    pub chall_dir: String,
    #[structopt(
        long,
        value_name = "path",
        default_value = "/var/lib/acme-redirect",
        env = "ACME_DATA_DIR"
    )]
    pub data_dir: String,
    #[structopt(long, default_value=LETSENCRYPT, env="ACME_URL")]
    pub acme_url: String,
    #[structopt(long, env = "ACME_EMAIL")]
    pub acme_email: Option<String>,
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Setup environment
    Setup(SetupArgs),
    /// Run the redirect daemon
    Daemon(DaemonArgs),
    /// Show the status of our certificates
    Status,
    /// Request new certificates if needed
    Renew(RenewArgs),
}

#[derive(Debug, StructOpt)]
pub struct SetupArgs {
    pub email: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct DaemonArgs {
    #[structopt(long = "chroot")]
    pub chroot: Option<String>,
    #[structopt(long = "user")]
    pub user: Option<String>,
    #[structopt(short = "B", long, default_value = "[::]:80", env = "ACME_BIND_ADDR")]
    pub bind_addr: String,
}

#[derive(Debug, StructOpt)]
pub struct RenewArgs {
    #[structopt(short = "n", long = "dry-run")]
    pub dry_run: bool,
}