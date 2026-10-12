export type ProxyType = "tcp" | "udp" | "http" | "https";

export interface Proxies {
    name: string;
    type: ProxyType;
    enabled?: boolean;
    localIp?: string;
    localPort?: number;
    remotePort?: number;
}

export interface Config {
    serverAddr?: string;
    serverPort?: number;
    proxies?: Proxies[];
    auth?: AuthClientConfig;
    transport?: ClientTransportConfig;
}

type AuthMethod = "token" | "oidc" | null;

export type additionalScopesType = "HeartBeats" | "NewWorkConns";

interface AuthClientConfig {
    method?: AuthMethod;
    token?: string | null;
    additionalScopes?: additionalScopesType[];
}

type Protocol = "tcp" | "kcp" | "quic" | "websocket" | "wss";

interface ClientTransportConfig {
    protocol?: Protocol;
    dialServerTimeout?: number;
    poolCount?: number;
    tcpMux?: boolean;
    tcpMuxKeepaliveInterval?: number;
    heartbeatInterval?: number;
    heartbeatTimeout?: number;
}

export enum ProxyOperation {
    "Add",
    "Edit",
}
