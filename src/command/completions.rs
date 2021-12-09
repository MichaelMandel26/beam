use crate::cli::Beam;
use anyhow::{anyhow, Result};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
pub struct Completions {
    #[structopt(help = "The shell to generate completions for")]
    shell: String,
}

impl Completions {
    pub fn run(&self) -> Result<()> {
        match self.shell.as_str() {
            "bash" => {
                Beam::clap().gen_completions_to("beam", Shell::Bash, &mut std::io::stdout());
            }
            "fish" => {
                Beam::clap().gen_completions_to("beam", Shell::Fish, &mut std::io::stdout());
            }
            "zsh" => {
                Beam::clap().gen_completions_to("beam", Shell::Zsh, &mut std::io::stdout());
            }
            _ => {
                return Err(anyhow!("Unsupported shell: {}", self.shell));
            }
        };
        Ok(())
    }
}
