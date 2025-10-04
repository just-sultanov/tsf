fn main() {
    let cli = tsf::parse_cli();
    let config = tsf::load_config(cli.config_path);
    let command = cli.command.unwrap();
    tsf::run(config, command);
}
