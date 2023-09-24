use env_logger::Env;

pub fn init_logger() {
    let env = Env::default()
        .filter_or("SFTP2API_LOG_LEVEL", "info")
        .write_style_or("SFTP2API_LOG_STYLE", "always");
    env_logger::init_from_env(env)
}
