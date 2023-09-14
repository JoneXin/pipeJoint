export type ProxyStruct = {
    source_ip: string;
    source_port: number;
    target_ip: string;
    target_port: number;
    protocol: string;
    status?: string;
    key?: string;
};
