use std::io;

use crate::cli::Beam;
use anyhow::{anyhow, Result};
use clap::{IntoApp, Parser};
use clap_complete::{generate, Shell};

#[derive(Debug, Parser)]
pub struct Completions {
    #[clap(help = "The shell to generate completions for")]
    shell: String,
}

impl Completions {
    pub fn run(&self) -> Result<()> {
        let cmd = &mut Beam::command();
        match self.shell.as_str() {
            "bash" => {
                generate(
                    Shell::Bash,
                    cmd,
                    cmd.get_name().to_string(),
                    &mut io::stdout(),
                );
            }
            "fish" => {
                generate(
                    Shell::Fish,
                    cmd,
                    cmd.get_name().to_string(),
                    &mut io::stdout(),
                );
            }
            "zsh" => {
                generate(
                    Shell::Zsh,
                    cmd,
                    cmd.get_name().to_string(),
                    &mut io::stdout(),
                );
            }
            _ => {
                return Err(anyhow!("Unsupported shell: {}", self.shell));
            }
        };
        Ok(())
    }
}
