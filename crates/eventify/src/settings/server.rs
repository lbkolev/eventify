use clap::Args;

#[derive(Args, Clone, Debug)]
#[group(skip)]
pub(crate) struct ServerSettings {
    #[arg(
        long = "server.enabled",
        env = "EVENTIFY_SERVER_ENABLED",
        help = "Toggler enabling|disabling the HTTP-API server",
        action
    )]
    pub(crate) server_enabled: bool,

    #[arg(
        long = "server.threads",
        env = "EVENTIFY_SERVER_THREADS",
        help = "The number of threads to use for the API server",
        default_value_t = num_cpus::get(),
    )]
    pub(crate) server_threads: usize,

    #[arg(
        long = "server.host",
        env = "EVENTIFY_SERVER_HOST",
        help = "The host to run the HTTP-API server on",
        default_value = "127.0.0.1"
    )]
    pub(crate) host: String,

    #[arg(
        long = "server.port",
        env = "EVENTIFY_SERVER_PORT",
        help = "The port to run the HTTP-API server on",
        default_value_t = 6969,
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    pub(crate) port: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::env::{remove_var, set_var};

    // as env vars are global resource and tests by default are ran in parallel
    // we need to make sure that we run them in serial mode so they don't interfere with one another
    use serial_test::serial;

    // A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[clap(flatten)]
        args: T,
    }

    #[test]
    #[serial]
    fn test_server_settings_default_values() {
        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
        assert!(!args.server_enabled);
        assert_eq!(args.server_threads, num_cpus::get());
        assert_eq!(args.host, "127.0.0.1");
        assert_eq!(args.port, 6969);
    }

    #[test]
    #[serial]
    fn test_server_settings_env_values() {
        set_var("EVENTIFY_SERVER_ENABLED", "true");
        set_var("EVENTIFY_SERVER_THREADS", "1");
        set_var("EVENTIFY_SERVER_HOST", "localhost");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from(["run"]).args;
        assert!(args.server_enabled);
        assert_eq!(args.server_threads, 1);
        assert_eq!(args.host, "localhost");
        assert_eq!(args.port, 1234);

        remove_var("EVENTIFY_SERVER_ENABLED");
        remove_var("EVENTIFY_SERVER_THREADS");
        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_server_settings_args_precedence() {
        set_var("EVENTIFY_SERVER_THREADS", "1");
        set_var("EVENTIFY_SERVER_HOST", "localhost");
        set_var("EVENTIFY_SERVER_PORT", "1234");

        let args = CommandParser::<ServerSettings>::parse_from([
            "run",
            "--server.enabled",
            "--server.threads",
            "2",
            "--server.host",
            "1.2.3.4",
            "--server.port",
            "5678",
        ])
        .args;

        assert!(args.server_enabled);
        assert_eq!(args.server_threads, 2);
        assert_eq!(args.host, "1.2.3.4");

        remove_var("EVENTIFY_SERVER_THREADS");
        remove_var("EVENTIFY_SERVER_HOST");
        remove_var("EVENTIFY_SERVER_PORT");
    }
}
