use crate::Lazy;
use crate::config::SETTINGS;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

/// Registro de trazas y eventos de la aplicación.
///
/// Para aumentar el rendimiento, un subproceso dedicado utiliza un sistema de
/// escritura sin bloqueo (*non-blocking writer*) que actúa periódicamente en
/// vez de enviar cada traza o evento al instante. Si el programa termina
/// abruptamente (por ejemplo, por un `panic!` o un `std::process::exit`), es
/// posible que algunas trazas o eventos no se envíen.
///
/// Puesto que las trazas o eventos registrados poco antes de la caída de una
/// aplicación suelen ser importantes para diagnosticar la causa del fallo, con
/// `Lazy<WorkerGuard>` se garantiza que todos los registros almacenados se
/// enviarán antes de terminar la ejecución.

pub static TRACING: Lazy<WorkerGuard> = Lazy::new(|| {
    let env_filter = EnvFilter::try_new(&SETTINGS.log.tracing)
        .unwrap_or(EnvFilter::new("Info"));

    let rolling = SETTINGS.log.rolling.to_lowercase();
    let (non_blocking, guard) = match rolling.as_str() {
        "stdout" => tracing_appender::non_blocking(
            std::io::stdout()
        ),
        _ => tracing_appender::non_blocking({
            let path = &SETTINGS.log.path;
            let prefix = &SETTINGS.log.prefix;
            match rolling.as_str() {
                "daily"    => tracing_appender::rolling::daily(path, prefix),
                "hourly"   => tracing_appender::rolling::hourly(path, prefix),
                "minutely" => tracing_appender::rolling::minutely(path, prefix),
                "endless"  => tracing_appender::rolling::never(path, prefix),
                _ => {
                    println!(
                        "Rolling value \"{}\" not valid. {}. {}.",
                        SETTINGS.log.rolling,
                        "Using \"daily\"",
                        "Check the settings file",
                    );
                    tracing_appender::rolling::daily(path, prefix)
                }
            }
        })
    };
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .with_ansi(rolling.as_str() == "stdout");
    match SETTINGS.log.format.to_lowercase().as_str() {
        "json"    => subscriber.json().init(),
        "full"    => subscriber.init(),
        "compact" => subscriber.compact().init(),
        "pretty"  => subscriber.pretty().init(),
        _ => {
            println!(
                "Tracing format \"{}\" not valid. {}. {}.",
                SETTINGS.log.format,
                "Using \"Full\"",
                "Check the settings file",
            );
            subscriber.init();
        }
    }

    guard
});