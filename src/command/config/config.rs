// use crate::utils::config::Config;
// use crate::ConfigOpts;
// use anyhow::{ensure, Result};

// pub fn config(cfg: ConfigOpts) -> Result<()> {
//     ensure!(cfg != Default::default(), "No config options specified");

//     if cfg.reset {
//         let config = Config::default();
//         config.write()?;
//         println!("Config successfully reset");
//         return Ok(());
//     }

//     let mut config = Config::get()?.unwrap_or_default();

//     if let Some(username) = cfg.username {
//         config.username = Some(username);
//     }

//     if let Some(cache_ttl) = cfg.cache_ttl {
//         config.cache_ttl = Some(cache_ttl);
//     }

//     if let Some(proxy) = cfg.proxy {
//         config.proxy = Some(proxy);
//     }

//     if let Some(auth) = cfg.auth {
//         config.auth = Some(auth);
//     }

//     config.write()?;

//     println!("Config successfully updated");

//     Ok(())
// }
