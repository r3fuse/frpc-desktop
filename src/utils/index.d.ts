interface Proxies{
    name:string,
    type:ProxyType,
    enabled?:boolean,
    localIp?:string,
    localPort?:number,
    remotePort?:number
}

interface Config{
    serverAddr:string,
    serverPort:number,
    proxies:Proxies[]
}

type ProxyType = "tcp"|"udp"|"http"|"https"