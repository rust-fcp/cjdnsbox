extern crate cjdnsbox;

use std::io::Read;
use std::str;

use cjdnsbox::{Controller,run};

macro_rules! config_tpl {
    () => { r#"
{{
    "privateKey": "9ef9aceb52dac9dc1022e79093e8611f58ef2eadfb930a4cbcc8baab58fc948b",
    "publicKey": "1kw05pc2xkxs7vwsfb1ct7kjzg0cshuwvm2qx3uk5s030k8nb4s0.k",
    "ipv6": "fdfe:3b03:cc12:4025:80bb:2c3e:72ec:280f",
    "authorizedPasswords":
    [
        {{"password": "vmf9crb0c6b813t44l2zbc53s679fj6", "user": "default-login"}}
    ],
    "admin":
    {{
        "bind": "127.0.0.1:{admin_port}",
        "password": "NONE"
    }},
    "interfaces":
    {{
        "UDPInterface":
        [
            {{
                "bind": "0.0.0.0:36312",
                "connectTo": {{}}
            }},
            {{
                "bind": "[::]:36312",
                "connectTo": {{}}
            }}
        ]
,
        "ETHInterface":
        [
            {{
                "bind": "all",
                "beacon": 2,
                "connectTo": {{}}
            }}
        ]
    }},
    "router":
    {{
        "supernodes": []
        "interface":
        {{
            "type": "TUNInterface"
        }},
        "ipTunnel":
        {{
            "allowedConnections": [],
            "outgoingConnections": []
        }}
    }},
    "security":
    [
        {{ "setuser": "nobody", "keepNetAdmin": 1 }},
        {{ "chroot": "/var/run/" }},
        {{ "nofiles": 0 }},
        {{ "noforks": 1 }},
        {{ "seccomp": 1 }},
        {{ "setupComplete": 1 }}
    ],
    "logging": {{}},
    "noBackground":0
}}
"# }
}

macro_rules! format_config {
    ( $($args:tt)* ) => { format!(config_tpl!(), $($args)* ) }
}

#[test]
fn base1() {
    let config = format_config!(admin_port=12345);
    let mut ctrl = run("/home/cjdns/cjdns/cjdroute", &config, "../target/debug/libcjdnsbox_hook.so").unwrap();
}
