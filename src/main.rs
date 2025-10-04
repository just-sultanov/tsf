fn main() {
    let cli = tsf::cli::parse();
    tsf::logger::init(cli.debug);
    let config = tsf::config::load(cli.config_path);
    let command = cli.command.unwrap();
    tsf::cli::run(config, command);
}
