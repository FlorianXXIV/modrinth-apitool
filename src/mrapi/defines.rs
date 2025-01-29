use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum VT {
    RELEASE,
    BETA,
    ALPHA,
}

impl VT {
    pub fn to_string(&self) -> String {
        match self {
            Self::RELEASE => String::from_str("release").expect("from_str"),
            Self::BETA => String::from_str("beta").expect("from_str"),
            Self::ALPHA => String::from_str("alpha").expect("from_str"),
        }
    }
}
impl<'de> Deserialize<'de> for VT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            match String::deserialize(deserializer)?.as_str() {
                "release" | "RELEASE" => Ok(VT::RELEASE),
                "beta" | "BETA" => Ok(VT::BETA),
                "alpha" | "ALPHA" => Ok(VT::ALPHA),
                _ => Err(serde::de::Error::custom("Expected either fabric, quilt or neoforge"))
            }
    }
}

impl FromStr for VT {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" | "RELEASE" => Ok(Self::RELEASE),
            "beta" | "BETA" => Ok(Self::BETA),
            "alpha" | "ALPHA" => Ok(Self::ALPHA),
            _ => Err("invalid version type".to_string()),
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum LOADER {
    FABRIC,
    QUILT,
    NEOFORGE
}

impl<'de> Deserialize<'de> for LOADER {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            match String::deserialize(deserializer)?.as_str() {
                "fabric" | "FABRIC" => Ok(LOADER::FABRIC),
                "quilt" | "QUILT" => Ok(LOADER::QUILT),
                "neoforge" | "NEOFORGE" => Ok(LOADER::NEOFORGE),
                _ => Err(serde::de::Error::custom("Expected either fabric, quilt or neoforge"))
            }
    }
}

impl LOADER {
    pub fn to_string(&self) -> String {
        match self {
            Self::FABRIC => "fabric".to_string(),
            Self::QUILT => "quilt".to_string(),
            Self::NEOFORGE => "neoforge".to_string()
        }
    }
}

impl FromStr for LOADER {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fabric" | "FABRIC" => Ok(Self::FABRIC),
            "neoforge" | "NEOFORGE" => Ok(Self::NEOFORGE),
            "quilt" | "QUILT" => Ok(Self::QUILT),
            "forge" | "FORGE" => Err("This tool does not support Forge".to_string()),
            _ => Err("Unknown Modloader".to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResp {
    pub hits: Vec<Value>,
    offset: i32,
    limit: i32,
    total_hits: i32,
}

//A specific version of a project

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub game_versions: Vec<String>,
    pub loaders: Vec<LOADER>,
    pub name: String,
    pub version_number: String,
    pub downloads: u32,
    pub version_type: VT,
    pub files: Vec<ApiFile>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiFile {
    pub url: String,
    pub filename: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    project_id: String,
    dependency_type: String,
}


//A modrinth Project, this can be a mod, modpack, resourcepack or shader
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    
}
