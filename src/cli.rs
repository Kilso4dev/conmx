use clap::App;
use std::str::FromStr;
use std::net::{ 
    IpAddr,
    Ipv6Addr,
};

use crate::err::ConmxErr;

pub enum CliOpts {
    Unvalidated(CliOptValues),
    Validated(CliOptValues),
}

pub struct CliOptValues {
    pub node_ip: IpAddr,
    pub version: String,
}

impl CliOpts {
    pub fn parse() -> Result<CliOpts, ConmxErr> {
        let m = App::new(format!("{} Controller software", "ConMX"))
            .version(crate_version!())
            .author(crate_authors!())
            .about("run visualization Software, usage: rnet <TargetAddr>")
            .get_matches();

        let node_ip = match m.value_of("node-ip") {
            Some(ip) => IpAddr::from_str(ip)
                .map_err(|e| ConmxErr::Net(format!("Ip \"{}\" not valid ({})", ip, e)))?,
            None => IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1)), // Local address
        };

        Ok(
            CliOpts::Unvalidated(
                CliOptValues {
                    node_ip,
                    version: String::from(crate_version!()),
                }
            )
        )
    }

    pub fn validate(self) -> Self {
        // TODO( Implement validation behavior )
        match self {
            CliOpts::Unvalidated(v) => CliOpts::Validated(v),
            val => val,
        }
    }
}


