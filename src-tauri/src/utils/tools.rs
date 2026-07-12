use tauri::{AppHandle};
use tauri_plugin_shell::process::{ CommandChild};
use std::{io::{Error, Write}, sync::Mutex};
use tauri::{path::BaseDirectory,Manager};
use crate::utils::proxies::{Config,Proxies};
use std::fs;

pub struct AppState {
    pub frpc_process: Mutex<Option<CommandChild>>,
}

pub fn has_frpc_process(app_handle:&AppHandle) -> bool {
    let state = app_handle.state::<AppState>();
    let process_guard = state.frpc_process.lock().unwrap();
    println!("frpc process exists: {}", process_guard.is_some());  
    process_guard.is_some()
}

pub fn mget_resource_path(app_handle: &AppHandle)->Result<String,String>{
    let resource_path = 
        app_handle.path()
            .resolve("resources/frpc.toml", BaseDirectory::Resource)
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|err| err.to_string());
    
    return resource_path;
}


//生成格式化文件
pub fn generate_config(data:Proxies)->Result<String,String>{
    let toml = toml::to_string(&data).expect("转换失败");
    let final_config: String = format!("[[proxies]]\n{}\n",toml);
    println!("{:?}",final_config);
    Ok(final_config)
}

//删除配置
pub fn delete_proxy( name:String,config:&mut Config )->Result<(),String>{
    for (index,item) in config.proxies.iter().enumerate(){
        if item.name == name {
            println!("匹配结果:位于{},内容是：{:?}",index,item);
            config.proxies.remove(index);
            println!("处理后：{:?}",config);
            return Ok(());
        }
    }
    println!("处理后：{:?}",config.proxies);
    return Err("修改失败".to_string());
}

//修改配置
pub fn change_proxy( name:String,config:&mut Config,new_proxy:Proxies )->Result<(),String>{
    for (index,item) in config.proxies.iter().enumerate(){
        if item.name == name {
            println!("匹配结果:位于{},内容是：{:?}",index,item);
            config.proxies[index] = new_proxy;
            println!("处理后：{:?}",config.proxies);
            return Ok(());
        }
    }
    println!("处理后：{:?}",config.proxies);
    return Err("修改失败".to_string());
}

//解析配置
pub async fn parsing_config()->Result<Config,Box<dyn std::error::Error>>{
    println!("开始解析");
    let s = fs::read_to_string("F:\\rwx\\test.toml")?;
    println!("文件内容是：{}",s);
    let config = toml::from_str::<Config>(&s)?;
    println!("{:?}",config);
    Ok(config)
}


pub async fn write_proxy(data:&Config,file_path:String)->Result<(),Error>{
    let mut f = fs::File::options().truncate(true).write(true).open(file_path)?;
    let toml = toml::to_string(data).expect("写入前，转换失败");
    f.write_all(toml.as_bytes())
} 
