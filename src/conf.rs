// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file to details.
//
// src/conf.rs
// This module handles configuration file parsing and loading.

use std::collections::{HashMap, HashSet};
use serde::Deserialize;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = 'kebab-case')]
enum CertContent {
    CertOnly,
    KeyOnly,
    CertKey,
    CertChainOnly,
    CertChainKey,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum CertType {
    PEM,
    DER,
    PKCS7,
    PKCS12,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
struct AcmeServer {
    host: String,
    port: u16,
    url: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
enum ConfigAcmeServer {
    Ref(String),
    Inline(AcmeServer),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = 'kebab-case')]
enum ChallengeType {
    Http01, // TODO HTTP-01 Chanllenge
    Dns01, // TODO DNS-01 Chanllenge
    TlsAlpn, // TODO TLS-ALPN Chanllenge
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum DnsType {
    SystemDNS,
    SoftwareDNS(String),
    DoT(String),
    DoH(String),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum ReloadType {
    CLI(String),
    UnixSocket(String),
    NetSocket(u16, String),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
enum UserAndGroup {
    ID(u32),
    Name(String),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
struct FileSave {
    path: String,
    mode: Option<String>,
    gid: Option<UserAndGroup>,
    uid: Option<UserAndGroup>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
struct CentOutput {
    cert_file: FileSave,
    cert_type: CertType,
    cert_content: CertContent,
    from_server: ConfigAcmeServer,
    challenge_type: ChallengeType,
    dns_type: DnsType,
    renew_threshold: String,
    reload_type: Option<ReloadType>,
    log_file: Option<FileSave>,
}

#[derive(Deserialize, Debug)] 
struct ConfigFile {
    user_id: UserAndGroup,
    check_interval: Option<String>,
    log_file: Option<FileSave>,
    acme_servers: Option<HashMap<String, AcmeServer>>,
    cert_output: HashMap<String, CentOutput>,
}

fn parse_config(config_path: &str) -> Result<ConfigFile, String> {
    match std::fs::read_to_string(config_path) {
        Ok(json_str) => {
            match serde_json::from_str(&json_str) {
                Ok(parsed_config) => Ok(parsed_config),
                Err(e) => Err(format!("Deserialization error: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to read configuration file: {}", e))
    }
}
