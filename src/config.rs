#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub private_key: String,

    #[clap(long, env)]
    pub public_key: String,

    #[clap(long, env, default_value = "80")]
    pub port: u16,
}
