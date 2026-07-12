export interface Proxies{
    name:string,
    type:ProxyType,
    enable?:boolean,
    localIp?:string,
    localPort?:number,
    remotePort?:number
}

export interface Config{
    serverAddr:string,
    serverPort:number,
    proxies:Proxies[]
}

export type ProxyType = "tcp"|"udp"|"http"|"https"