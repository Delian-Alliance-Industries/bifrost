use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
struct Kubeconfig {
    #[serde(default)]
    contexts: Vec<NamedContext>,
    #[serde(rename = "current-context", default)]
    current_context: Option<String>,
    #[serde(flatten)]
    other: serde_yaml_ng::Mapping,
}

#[derive(Debug, Serialize, Deserialize)]
struct NamedContext {
    name: String,
    #[serde(default)]
    context: Option<ContextDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContextDetails {
    #[serde(default)]
    cluster: Option<String>,
    #[serde(default)]
    namespace: Option<String>,
    #[serde(flatten)]
    other: serde_yaml_ng::Mapping,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "\
bifrost - cross the rainbow bridge between Kubernetes contexts

USAGE:
    bifrost [OPTIONS]

OPTIONS:
    -k, --kubeconfig <PATH>  Path to kubeconfig file
                             (overrides $KUBECONFIG and default)
    -h, --help               Print this help and exit
    -V, --version            Print version and exit

ENV:
    KUBECONFIG       Path(s) to kubeconfig (first is used if colon-separated).
                     Defaults to ~/.kube/config.
";

fn die(msg: impl std::fmt::Display) -> ! {
    eprintln!("{msg}");
    process::exit(1);
}

fn default_kubeconfig_path() -> PathBuf {
    if let Ok(path) = std::env::var("KUBECONFIG") {
        PathBuf::from(path.split(':').next().unwrap_or(&path))
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| die("could not determine home directory"))
            .join(".kube")
            .join("config")
    }
}

fn parse_args() -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
    let mut kubeconfig: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print!("{HELP}");
                process::exit(0);
            }
            "-V" | "--version" => {
                println!("bifrost {VERSION}");
                process::exit(0);
            }
            "-k" | "--kubeconfig" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| die(format!("{arg} requires a path argument")));
                kubeconfig = Some(PathBuf::from(value));
            }
            _ if arg.starts_with("--kubeconfig=") => {
                kubeconfig = Some(PathBuf::from(&arg["--kubeconfig=".len()..]));
            }
            _ => {
                eprintln!("unknown argument: {arg}\n\n{HELP}");
                process::exit(2);
            }
        }
    }
    kubeconfig
}

fn main() {
    let path = parse_args().unwrap_or_else(default_kubeconfig_path);
    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
        die(format!(
            "Failed to read kubeconfig at {}: {e}",
            path.display()
        ))
    });

    let mut config: Kubeconfig = serde_yaml_ng::from_str(&content)
        .unwrap_or_else(|e| die(format!("Failed to parse kubeconfig: {e}")));

    if config.contexts.is_empty() {
        die("No contexts found in kubeconfig.");
    }

    let current = config.current_context.as_deref().unwrap_or("");

    cliclack::intro("bifrost - cross the rainbow bridge").ok();

    let mut select = cliclack::select(format!("Pick a context (current: {current})"));

    for ctx in &config.contexts {
        let hint = ctx.context.as_ref().map_or(String::new(), |d| {
            let cluster = d.cluster.as_deref().unwrap_or("?");
            let ns = d.namespace.as_deref().unwrap_or("default");
            format!("{cluster} / {ns}")
        });

        let label = if ctx.name == current {
            format!("{} *", ctx.name)
        } else {
            ctx.name.clone()
        };

        select = select.item(ctx.name.clone(), label, hint);
    }

    let selected: String = select.filter_mode().interact().unwrap_or_else(|_| {
        cliclack::outro("Cancelled.").ok();
        process::exit(0);
    });

    if selected == current {
        cliclack::outro(format!("Already on {selected}. No change.")).ok();
        return;
    }

    config.current_context = Some(selected.clone());

    let new_content = serde_yaml_ng::to_string(&config)
        .unwrap_or_else(|e| die(format!("Failed to serialize kubeconfig: {e}")));

    fs::write(&path, new_content)
        .unwrap_or_else(|e| die(format!("Failed to write kubeconfig: {e}")));

    cliclack::outro(format!("Switched to {selected}")).ok();
}
