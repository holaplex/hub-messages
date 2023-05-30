use hub_core::clap;

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct SmtpServerArgs {
    #[arg(long, env)]
    pub server: String,

    #[arg(long, env)]
    pub username: String,

    #[arg(long, env)]
    pub password: String,

    #[arg(long, env)]
    pub plaintext_port: u16,

    #[arg(long, env)]
    pub pool_size: u16,
}
