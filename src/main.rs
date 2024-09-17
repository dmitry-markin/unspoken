use anyhow::{anyhow, Context as _};
use clap::Parser;
use colored::Colorize as _;
use dirs::home_dir;
use std::{
    env, fs,
    io::{self, Write as _},
    path::PathBuf,
};
use unspoken::{ChatClient, ChatClientConfig};

/// OpenAI chat API command line client.
///
/// Command line options override config file.
#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// API url. Example: "https://models.inference.ai.azure.com/".
    #[arg(short, long)]
    url: Option<String>,

    /// Model. Example: "gpt-4o".
    #[arg(short, long)]
    model: Option<String>,

    /// System message to initialize the model. Example: "You are a helpful assistant."
    #[arg(short, long)]
    system: Option<String>,

    /// Config file location. Default: $HOME/.config/unspoken.toml.
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    api_key: Option<String>,
    url: Option<String>,
    model: Option<String>,
    system_message: Option<String>,
}

struct AppConfiguration {
    api_key: String,
    api_url: String,
    model: String,
    system_message: Option<String>,
}

impl AppConfiguration {
    fn init(args: Args) -> anyhow::Result<Self> {
        let Args {
            url,
            model,
            system,
            config,
        } = args;

        let config: Option<Config> = if let Some(config_path) = config {
            // Try reading CLI-provided config file first.
            Some(
                toml::from_str(&fs::read_to_string(config_path.clone()).with_context(|| {
                    anyhow!(
                        "Failed to read config file {}",
                        config_path
                            .to_str()
                            .expect("to have only unicode characters in path")
                    )
                })?)
                .context("Failed to parse config file {config_path}")?,
            )
        } else {
            // If there is $HOME, try reading config from standard path.
            if let Some(config_path) = home_dir().map(|home| home.join(".config/unspoken.toml")) {
                match fs::read_to_string(config_path.clone()) {
                    Ok(string) => Ok(toml::from_str(&string).with_context(|| {
                        anyhow!(
                            "Failed to parse config file {}",
                            config_path
                                .to_str()
                                .expect("to have only unicode characters in path")
                        )
                    })?),
                    Err(error) => match error.kind() {
                        // Missing config in $HOME is not an error.
                        io::ErrorKind::NotFound => Ok(None),
                        _ => Err(error).context("Failed to read config file {config_path}"),
                    },
                }?
            } else {
                None
            }
        };

        let api_key = env::var("OPENAI_API_KEY").or_else(|_| {
            config
                .as_ref()
                .map(|c| c.api_key.clone())
                .flatten()
                .ok_or(anyhow!("Set `api_key` in config or `OPENAI_API_KEY` env."))
        })?;

        let api_url = url
            .or_else(|| config.as_ref().map(|c| c.url.clone()).flatten())
            .unwrap_or_else(|| String::from("https://models.inference.ai.azure.com/"));

        let model = model
            .or_else(|| config.as_ref().map(|c| c.model.clone()).flatten())
            .unwrap_or_else(|| String::from("gpt-4o"));

        let system_message =
            system.or_else(|| config.as_ref().map(|c| c.system_message.clone()).flatten());

        Ok(Self {
            api_key,
            api_url,
            model,
            system_message,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let AppConfiguration {
        api_key,
        api_url,
        model,
        system_message,
    } = AppConfiguration::init(Args::parse())?;

    let mut chat = ChatClient::new(
        api_key,
        ChatClientConfig {
            api_url,
            model,
            system_message,
        },
    );

    let you = "You:".bold().red();
    let assistant = "Assistant:".bold().green();

    print!("{} ", you);
    io::stdout().flush()?;

    for line in std::io::stdin().lines() {
        match chat.ask(line?) {
            Ok(response) => {
                print!("\n{} {response}\n\n{} ", assistant, you);
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".yellow(), e.to_string().yellow());
                print!("{} ", you);
            }
        }
        io::stdout().flush()?;
    }

    println!("");

    Ok(())
}
