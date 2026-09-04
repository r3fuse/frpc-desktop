use std::path::PathBuf;
use std::fs;
use anyhow::{Result};
use crate::utils::proxies::{Config, Proxies};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct ConfigStore{
    path:PathBuf,
    data:Config
}

impl ConfigStore {
    pub fn new() -> Result<Self>{
        let dir = directories::ProjectDirs::from("com", "frp", "frpGui")
            .ok_or_else(||anyhow::anyhow!("Failed to get project dirs"))?;
        let config_dir = dir.config_dir();
        fs::create_dir_all(config_dir)?;
        let path = config_dir.join("config.toml");

        let data = if path.exists() {
            let content = fs::read_to_string(&path)?;
            match toml::from_str(&content) {
                Ok(d)=>d,
                Err(e)=>{
                    println!("config file corrupted {}:{}",path.display(),e);
                    Config::default()
                }
            }
        } else {
            Config::default()
        };
        Ok(Self { path, data })
    }

    pub fn data(&self) -> &Config{
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Config{
        &mut self.data
    }

    pub fn save(&self) ->Result<()> {
        let content = toml::to_string_pretty(&self.data)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn add_proxy(&mut self,proxy:Proxies){
        self.data_mut().proxies.get_or_insert(Vec::new()).push(proxy);
    }

    pub fn delect_proxy(&mut self,name:String){
        if let Some(proxies) = self.data_mut().proxies.as_mut(){
            if let Some(index) = proxies.iter_mut().position(|p|p.name == name){
                proxies.remove(index);
            }
        }
    }

    pub fn change_proxy(&mut self,name:String,new_proxy:Proxies){
        if let Some(proxies) = self.data_mut().proxies.as_mut(){
            if let Some(proxy) = proxies.iter_mut().find(|p|p.name == name){
                *proxy = new_proxy;
            }
        }
    }

    pub fn change_proxy_activation_status(&mut self,name:String){
        if let Some(proxies )= self.data_mut().proxies.as_mut(){
            if let Some(proxy) = proxies.iter_mut().find(|p| p.name == name){
                let status = proxy.enabled.unwrap_or(true);
                proxy.enabled = Some(!status);
            }
        }
    }


}

