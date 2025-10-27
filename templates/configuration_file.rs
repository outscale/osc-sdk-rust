/*
Copyright (c) 2022 OUTSCALE SAS. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors
may be used to endorse or promote products derived from this software without
specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::apis::configuration::{AWSv4Key, Configuration};
use crate::apis::middleware::{BackoffParams, LimiterParams};
use home::home_dir;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ConfigurationFile(pub HashMap<String, Profile>);

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub x509_client_cert: Option<String>,
    pub x509_client_key: Option<String>,
    pub protocol: Option<String>,
    pub method: Option<String>,
    pub region: Option<String>,
    pub endpoints: Option<Endpoint>,
    #[serde(flatten)]
    pub backoff_params: BackoffParams,
    #[serde(skip)]
    pub limiter_params: LimiterParams,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub api: Option<String>,
    pub fcu: Option<String>,
    pub lbu: Option<String>,
    pub eim: Option<String>,
    pub icu: Option<String>,
    pub oos: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ConfigurationFileError {
    CannotGetDefaultPath,
    ProfileNotFound,
}

impl fmt::Display for ConfigurationFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigurationFileError::CannotGetDefaultPath => {
                write!(f, "cannot find default configuration file path")
            }
            ConfigurationFileError::ProfileNotFound => write!(f, "profile not found"),
        }
    }
}

impl Error for ConfigurationFileError {}

impl ConfigurationFile {
    pub fn default_path() -> Result<PathBuf, ConfigurationFileError> {
        let mut path = match home_dir() {
            Some(p) => p,
            None => return Err(ConfigurationFileError::CannotGetDefaultPath),
        };
        path.push(".osc");
        path.push("config.json");
        Ok(path)
    }

    pub fn load_default() -> Result<ConfigurationFile, Box<dyn Error>> {
        let path = ConfigurationFile::default_path()?;
        ConfigurationFile::load(&path)
    }

    pub fn load(path: &PathBuf) -> Result<ConfigurationFile, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configuration_file = serde_json::from_reader(reader)?;
        Ok(configuration_file)
    }

    pub fn configuration<S: Into<String>>(
        &self,
        profile_name: S,
    ) -> Result<Configuration, Box<dyn Error>> {
        let profile_name = profile_name.into();
        let profile = match self.0.get(&profile_name) {
            Some(profile) => profile.clone(),
            None => return Err(Box::new(ConfigurationFileError::ProfileNotFound)),
        };

        let mut config = Configuration {
            client: super::middleware::ClientWithBackoff::new(
                reqwest::blocking::Client::new(),
                profile.backoff_params.clone(),
                profile.limiter_params.clone(),
            ),
            ..Default::default()
        };

        if let Some(ref region) = profile.region {
            config.base_path = format!("https://api.{}.outscale.com/api/v1", region);
        }

        if let Some(endpoints) = profile.endpoints {
            if let Some(api_endpoint) = endpoints.api {
                match profile.protocol {
                    Some(protocol) => {
                        config.base_path = format!("{}://{}", protocol, api_endpoint);
                    }
                    None => {
                        config.base_path = format!("https://{}", api_endpoint);
                    }
                }
            }
        };

        if let Some(access_key) = profile.access_key {
            if let Some(secret_key) = profile.secret_key {
                let region = match profile.region {
                    Some(r) => r.clone(),
                    None => "eu-west-2".to_string(),
                };
                config.aws_v4_key = Some(AWSv4Key {
                    access_key,
                    secret_key: secret_key.into(),
                    region,
                    service: "oapi".to_string(),
                });
            }
        }
        Ok(config)
    }
}
