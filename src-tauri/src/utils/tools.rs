use tauri::{AppHandle};
use tauri_plugin_shell::process::{ CommandChild};
use std::{ sync::Mutex};
use tauri::{path::BaseDirectory,Manager};
use crate::utils::proxies::{Config, Proxies};
use std::fs;
use anyhow::{Result};

pub struct AppState {
    pub frpc_process: Mutex<Option<CommandChild>>,
    pub config: Mutex<crate::config_store::ConfigStore>
}

pub fn has_frpc_process(app_handle:&AppHandle) -> bool {
    let state = app_handle.state::<AppState>();
    let process_guard = state.frpc_process.lock().unwrap();
    println!("frpc process exists: {}", process_guard.is_some());  
    process_guard.is_some()
}

pub fn get_resource_path(app_handle: &AppHandle)->Result<String,String>{
    let resource_path = 
        app_handle.path()
            .resolve("resources/frpc.toml", BaseDirectory::Resource)
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|err| err.to_string());
    println!("{:?}",resource_path.clone()?);
    
    return resource_path;
}


//生成格式化Proxies文件
pub fn generate_config(data:Proxies)->Result<String,String>{
    let toml = toml::to_string(&data).expect("转换失败");
    let final_config: String = format!("\n[[proxies]]\n{}",toml);
    println!("{:?}",final_config);
    Ok(final_config)
}

//生成格式化serverBase文件
pub fn generate_server_config(data:Config)->Result<String,String>{
    let toml = toml::to_string(&data).expect("转换失败");
    let final_config: String = format!("{}\n",toml);
    println!("{:?}",final_config);
    Ok(final_config)
}

//删除配置
pub fn delete_proxy( name:String,config:&mut Config )->Result<(),String>{
    if let Some(proxies) = config.proxies.as_mut(){
         for (index,item) in proxies.iter().enumerate(){
            if item.name == name {
                println!("匹配结果:位于{},内容是：{:?}",index,item);
                proxies.remove(index);
                println!("处理后：{:?}",config);
                return Ok(());
            }
        }
        println!("处理后：{:?}",config.proxies);
        return Err("修改失败".to_string());
    }
    return Err("修改失败".to_string());
}

//修改配置
pub fn change_proxy( name:String,config:&mut Config,new_proxy:Proxies )->Result<(),String>{
    if let Some(proxies) = config.proxies.as_mut(){
        if let Some(index) = proxies.iter().position(|item| item.name == name){
            proxies[index] = new_proxy;
            return Ok(());
        }
    }


    // for (index,item) in config.proxies.iter().enumerate(){
    //     if item.name == name && item.name != "" {
    //         println!("匹配结果:位于{},内容是：{:?}",index,item);
    //         config.proxies[index] = new_proxy;
    //         println!("处理后：{:?}",config.proxies);
    //         return Ok(());
    //     }
    // }
    println!("处理后：{:?}",config.proxies);
    return Err("修改失败".to_string());
}

//修改启用状态
pub fn change_proxy_activation_status(name:String,config:&mut Config)->Result<(),String>{
    if let Some(proxies) = config.proxies.as_mut(){
        if let Some(index) = proxies.iter().position(|item| item.name == name ){
            proxies[index].enabled = Some(!proxies[index].enabled.unwrap_or(true));
            return Ok(());
        }
    }


    // for item in config.proxies.iter_mut(){
    //     if item.name == name {
    //        let status = item.enabled.unwrap_or(true);
    //        item.enabled = Some(!status);
    //        return Ok(());
    //     }
    // }
    return Err("更改失败".to_string());
}

//解析配置
pub async fn parsing_config(app_handle: &AppHandle)->Result<Config,Box<dyn std::error::Error>>{
    let path = get_resource_path(app_handle)?;
    println!("开始解析");
    let s = fs::read_to_string(&path)?;
    println!("文件内容是：{}",s);
    println!("文件内容是否为空：{}",s.is_empty());
    if s.is_empty() {
        match init_server_base(app_handle).await {
            Ok(v)=>return Ok(v),
            Err(_)=>{
                panic!("初始化失败！");
            }
        };
    }
    let config = toml::from_str::<Config>(&s)?;
    println!("{:?}",config);
    Ok(config)
}


// pub async fn write_proxy(data:&Config,file_path:String)->Result<(),Error>{
//     let mut f = fs::File::options().truncate(true).write(true).open(file_path)?;
//     let toml = toml::to_string(data).expect("写入前，转换失败");
//     f.write_all(toml.as_bytes())
// }

pub async fn write_proxy(app_handle: &AppHandle,data:&mut Config)->Result<()>{
    let state = app_handle.state::<AppState>();
    let mut config = state.config.lock().unwrap();
    *config.data_mut() = data.clone();
    config.save()?;
    Ok(())
}

pub async fn check_name_is_exist(name:String,config:&mut Config)->bool{
    if let Some(proxies) = config.proxies.as_mut(){
        if let Some(_) = proxies.iter().position(|item| item.name == name) {
            return true;
        }
    }

    // for item in config.proxies.iter()  {
    //     if item.name == name {
    //          return true;
    //     }
    // }
    return false;
}

pub async fn change_server_config(app_handle: &AppHandle,config:&mut Config,new_config:Config)->Result<(),String>{
    // let path = get_resource_path(app_handle)?;
    config.server_addr = new_config.server_addr;
    config.server_port = new_config.server_port;
    config.auth = new_config.auth;
    println!("config:{:?}",config);
    // println!("new_config.auth:{:?}",new_config.auth);
    match write_proxy(app_handle,config).await{
        Ok(_)=>{
            println!("file write successfully!");
            return Ok(());
        },
        Err(e)=>{
            println!("file write failed:{}",e);
            return Err("写入失败".to_string());
        }
    }
}

async fn init_server_base(app_handle: &AppHandle)->Result<Config,String>{
    println!("start init_server_base");
    let mut config = Config{
        server_addr:"example.com".into(),
        server_port:3389,
        proxies:None,
        auth:None
    };
    match write_proxy(app_handle,&mut config).await {
        Ok(_)=>return Ok(config),
        Err(e)=> return Err(e.to_string())
    } 

}
