export interface Proxies{
    name:string,
    type:ProxyType,
    enable?:boolean,
    localIp?:string,
    localPort?:number,
    remotePort?:number
}

export interface Config{
    serverAddr?:string,
    serverPort?:number,
    proxies?:Proxies[],
    auth?:AuthClientConfig
}

type AuthMethod = "token"|"oidc"

interface AuthClientConfig{
    method:AuthMethod
    token?:string|null
}

export type ProxyType = "tcp"|"udp"|"http"|"https"