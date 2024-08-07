#[macro_export]
macro_rules! log_error {
    ($e:expr) => {
        if let Err(e) = $e {
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
    ($context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = format!("{:?}", ::anyhow::anyhow!(e).context($context));
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
    ($ok_context:expr, $err_context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = format!("{:?}", ::anyhow::anyhow!(e).context($err_context));
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        } else {
            tracing::info!($ok_context);
        }
    };
}

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}
