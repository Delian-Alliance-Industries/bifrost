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
    other: serde_yaml::Mapping,
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
    other: serde_yaml::Mapping,
}

fn kubeconfig_path() -> PathBuf {
    if let Ok(path) = std::env::var("KUBECONFIG") {
        PathBuf::from(path.split(':').next().unwrap_or(&path))
    } else {
        dirs::home_dir()
            .expect("could not determine home directory")
            .join(".kube")
            .join("config")
    }
}

fn main() {
    let path = kubeconfig_path();
    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Failed to read kubeconfig at {}: {e}", path.display());
        process::exit(1);
    });

    let mut config: Kubeconfig = serde_yaml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Failed to parse kubeconfig: {e}");
        process::exit(1);
    });

    if config.contexts.is_empty() {
        eprintln!("No contexts found in kubeconfig.");
        process::exit(1);
    }

    let current = config.current_context.clone().unwrap_or_default();

    cliclack::intro("bifrost - cross the rainbow bridge").expect("intro failed");

    let mut select = cliclack::select(format!("Pick a context (current: {current})"));

    for ctx in &config.contexts {
        let hint = match &ctx.context {
            Some(details) => {
                let cluster = details.cluster.as_deref().unwrap_or("?");
                let ns = details.namespace.as_deref().unwrap_or("default");
                format!("{cluster} / {ns}")
            }
            None => String::new(),
        };

        let label = if ctx.name == current {
            format!("{} *", ctx.name)
        } else {
            ctx.name.clone()
        };

        select = select.item(ctx.name.clone(), label, hint);
    }

    select = select.filter_mode();

    let selected: String = match select.interact() {
        Ok(s) => s,
        Err(_) => {
            cliclack::outro("Cancelled.").ok();
            process::exit(0);
        }
    };

    if selected == current {
        cliclack::outro(format!("Already on {selected}. No change.")).ok();
        return;
    }

    config.current_context = Some(selected.clone());

    let new_content = serde_yaml::to_string(&config).unwrap_or_else(|e| {
        eprintln!("Failed to serialize kubeconfig: {e}");
        process::exit(1);
    });

    fs::write(&path, new_content).unwrap_or_else(|e| {
        eprintln!("Failed to write kubeconfig: {e}");
        process::exit(1);
    });

    cliclack::outro(format!("Switched to {selected}")).ok();
}
