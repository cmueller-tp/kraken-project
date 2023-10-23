//! The gRPC part of the leech.
//!
//! In server mode, the leech has a grpc server running to receive requests from kraken.
//! If the connection drops or the leech can't send the data, it will be saved in the local
//! database and pushing the data to the server is tried regularly.
//!
//! In cli mode, the leech can push the results to kraken if desired.

use std::error::Error;
use std::net::SocketAddr;

use log::info;
use tonic::transport::Server;

use crate::backlog::Backlog;
use crate::config::Config;
use crate::rpc::attacks::Attacks;
use crate::rpc::rpc_attacks::req_attack_service_server::ReqAttackServiceServer;

pub mod attacks;
mod utils;

/// Missing docs are allowed, as the code gets auto generated by tonic
#[allow(missing_docs)]
pub mod rpc_attacks {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
    use std::ops::RangeInclusive;
    use std::str::FromStr;

    use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
    use tonic::Status;

    use crate::models::{DnsRecordType, DnsResult, HostAliveResult, TcpPortScanResult};
    use crate::modules::bruteforce_subdomains::BruteforceSubdomainResult;
    use crate::modules::dns::DnsRecordResult;
    use crate::rpc::rpc_attacks::shared::dns_record::Record;
    use crate::rpc::rpc_attacks::shared::{
        Aaaa, Address, DnsRecord, GenericRecord, Ipv4, Ipv6, Net, NetOrAddress, A,
    };

    pub mod shared {
        tonic::include_proto!("attacks.shared");
    }

    tonic::include_proto!("attacks");

    impl From<Ipv4Addr> for Ipv4 {
        fn from(value: Ipv4Addr) -> Self {
            Self {
                address: i32::from_le_bytes(value.octets()),
            }
        }
    }

    impl From<Ipv4> for Ipv4Addr {
        fn from(value: Ipv4) -> Self {
            let [a, b, c, d] = value.address.to_le_bytes();
            Ipv4Addr::new(a, b, c, d)
        }
    }

    impl From<Ipv6> for Ipv6Addr {
        fn from(value: Ipv6) -> Self {
            let [a, b, c, d, e, f, g, h] = value.part0.to_le_bytes();
            let [i, j, k, l, m, n, o, p] = value.part1.to_le_bytes();
            Ipv6Addr::from([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
        }
    }

    impl From<Ipv6Addr> for Ipv6 {
        fn from(value: Ipv6Addr) -> Self {
            let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = value.octets();
            let part0 = i64::from_le_bytes([a, b, c, d, e, f, g, h]);
            let part1 = i64::from_le_bytes([i, j, k, l, m, n, o, p]);

            Self { part0, part1 }
        }
    }

    impl From<Net> for IpNetwork {
        fn from(value: Net) -> Self {
            match value.net.unwrap() {
                shared::net::Net::Ipv4net(x) => {
                    let addr: Ipv4Addr = x.address.unwrap().into();
                    let [a, b, c, d] = x.netmask.to_le_bytes();

                    Self::V4(Ipv4Network::with_netmask(addr, Ipv4Addr::new(a, b, c, d)).unwrap())
                }
                shared::net::Net::Ipv6net(x) => {
                    let addr: Ipv6Addr = x.address.unwrap().into();
                    let [a, b, c, d, e, f, g, h] = x.netmask0.to_le_bytes();
                    let [i, j, k, l, m, n, o, p] = x.netmask1.to_le_bytes();

                    Self::V6(
                        Ipv6Network::with_netmask(
                            addr,
                            Ipv6Addr::from([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]),
                        )
                        .unwrap(),
                    )
                }
            }
        }
    }

    impl From<NetOrAddress> for IpNetwork {
        fn from(value: NetOrAddress) -> Self {
            match value.net_or_address.unwrap() {
                shared::net_or_address::NetOrAddress::Address(x) => {
                    let addr: IpAddr = x.into();
                    match addr {
                        IpAddr::V4(x) => Self::V4(Ipv4Network::from(x)),
                        IpAddr::V6(x) => Self::V6(Ipv6Network::from(x)),
                    }
                }
                shared::net_or_address::NetOrAddress::Net(x) => x.into(),
            }
        }
    }

    impl From<BruteforceSubdomainResult> for BruteforceSubdomainResponse {
        fn from(value: BruteforceSubdomainResult) -> Self {
            Self {
                record: Some(match value {
                    BruteforceSubdomainResult::A { source, target } => DnsRecord {
                        record: Some(Record::A(A {
                            source,
                            to: Some(target.into()),
                        })),
                    },
                    BruteforceSubdomainResult::Aaaa { source, target } => DnsRecord {
                        record: Some(Record::Aaaa(Aaaa {
                            source,
                            to: Some(target.into()),
                        })),
                    },
                    BruteforceSubdomainResult::Cname { source, target } => DnsRecord {
                        record: Some(Record::Cname(GenericRecord { source, to: target })),
                    },
                }),
            }
        }
    }

    impl From<DnsRecordResult> for DnsResolutionResponse {
        fn from(value: DnsRecordResult) -> Self {
            Self {
                record: Some(match value {
                    DnsRecordResult::A { source, target } => DnsRecord {
                        record: Some(Record::A(A {
                            source,
                            to: Some(target.into()),
                        })),
                    },
                    DnsRecordResult::Aaaa { source, target } => DnsRecord {
                        record: Some(Record::Aaaa(Aaaa {
                            source,
                            to: Some(target.into()),
                        })),
                    },
                    DnsRecordResult::CAA { source, target } => DnsRecord {
                        record: Some(Record::Caa(GenericRecord { source, to: target })),
                    },
                    DnsRecordResult::Cname { source, target } => DnsRecord {
                        record: Some(Record::Cname(GenericRecord { source, to: target })),
                    },
                    DnsRecordResult::Mx { source, target } => DnsRecord {
                        record: Some(Record::Mx(GenericRecord { source, to: target })),
                    },
                    DnsRecordResult::Tlsa { source, target } => DnsRecord {
                        record: Some(Record::Tlsa(GenericRecord { source, to: target })),
                    },
                    DnsRecordResult::Txt { source, target } => DnsRecord {
                        record: Some(Record::Txt(GenericRecord { source, to: target })),
                    },
                }),
            }
        }
    }

    impl From<SocketAddr> for TcpPortScanResponse {
        fn from(value: SocketAddr) -> Self {
            let address = match value {
                SocketAddr::V4(v) => Address {
                    address: Some(shared::address::Address::Ipv4((*v.ip()).into())),
                },
                SocketAddr::V6(v) => Address {
                    address: Some(shared::address::Address::Ipv6((*v.ip()).into())),
                },
            };

            Self {
                address: Some(address),
                port: value.port() as u32,
            }
        }
    }

    impl From<IpAddr> for HostsAliveResponse {
        fn from(value: IpAddr) -> Self {
            Self {
                host: Some(value.into()),
            }
        }
    }

    impl From<Address> for IpAddr {
        fn from(value: Address) -> Self {
            let Address { address } = value;
            match address.unwrap() {
                shared::address::Address::Ipv4(v) => IpAddr::from(Ipv4Addr::from(v)),
                shared::address::Address::Ipv6(v) => IpAddr::from(Ipv6Addr::from(v)),
            }
        }
    }

    impl From<IpAddr> for Address {
        fn from(value: IpAddr) -> Self {
            Self {
                address: Some(match value {
                    IpAddr::V4(addr) => shared::address::Address::Ipv4(addr.into()),
                    IpAddr::V6(addr) => shared::address::Address::Ipv6(addr.into()),
                }),
            }
        }
    }

    impl TryFrom<PortOrRange> for RangeInclusive<u16> {
        type Error = Status;

        fn try_from(value: PortOrRange) -> Result<Self, Self::Error> {
            let value = value
                .port_or_range
                .ok_or_else(|| Status::invalid_argument("Missing inner field `port_or_range`"))?;
            let try_port = |port| {
                let port = u16::try_from(port)
                    .map_err(|_| Status::invalid_argument(format!("Port {port} is too big")))?;
                if port == 0 {
                    return Err(Status::invalid_argument("Ports can't be zero"));
                }
                Ok(port)
            };
            match value {
                port_or_range::PortOrRange::Single(port) => {
                    let port = try_port(port)?;
                    Ok(port..=port)
                }
                port_or_range::PortOrRange::Range(range) => {
                    let start = try_port(range.start)?;
                    let end = try_port(range.end)?;
                    if start <= end {
                        Ok(start..=end)
                    } else {
                        Err(Status::invalid_argument(
                            "Range start is bigger than its end",
                        ))
                    }
                }
            }
        }
    }

    impl From<DnsResult> for BacklogDnsResult {
        fn from(value: DnsResult) -> Self {
            Self {
                attack_uuid: value.attack.to_string(),
                record: match value.dns_record_type {
                    DnsRecordType::A => Some(DnsRecord {
                        record: Some(Record::A(A {
                            source: value.source,
                            to: Some(Ipv4Addr::from_str(&value.destination).unwrap().into()),
                        })),
                    }),
                    DnsRecordType::Aaaa => Some(DnsRecord {
                        record: Some(Record::Aaaa(Aaaa {
                            source: value.source,
                            to: Some(Ipv6Addr::from_str(&value.destination).unwrap().into()),
                        })),
                    }),
                    DnsRecordType::Cname => Some(DnsRecord {
                        record: Some(Record::Cname(GenericRecord {
                            source: value.source,
                            to: value.destination,
                        })),
                    }),
                    DnsRecordType::Caa => Some(DnsRecord {
                        record: Some(Record::Caa(GenericRecord {
                            source: value.source,
                            to: value.destination,
                        })),
                    }),
                    DnsRecordType::Mx => Some(DnsRecord {
                        record: Some(Record::Mx(GenericRecord {
                            source: value.source,
                            to: value.destination,
                        })),
                    }),
                    DnsRecordType::Tlsa => Some(DnsRecord {
                        record: Some(Record::Tlsa(GenericRecord {
                            source: value.source,
                            to: value.destination,
                        })),
                    }),
                    DnsRecordType::Txt => Some(DnsRecord {
                        record: Some(Record::Txt(GenericRecord {
                            source: value.source,
                            to: value.destination,
                        })),
                    }),
                },
            }
        }
    }

    impl From<Vec<DnsResult>> for BacklogDnsRequest {
        fn from(value: Vec<DnsResult>) -> Self {
            let mut entries: Vec<BacklogDnsResult> = Vec::new();
            entries.reserve(value.len());

            for e in value {
                entries.push(e.into());
            }

            Self { entries }
        }
    }

    impl From<TcpPortScanResult> for BacklogTcpPortScanResult {
        fn from(value: TcpPortScanResult) -> Self {
            let address = match value.address {
                IpNetwork::V4(v) => Address {
                    address: Some(shared::address::Address::Ipv4((v.ip()).into())),
                },
                IpNetwork::V6(v) => Address {
                    address: Some(shared::address::Address::Ipv6((v.ip()).into())),
                },
            };

            Self {
                attack_uuid: value.attack.to_string(),
                address: Some(address),
                port: value.port as u32, // TODO
            }
        }
    }

    impl From<Vec<TcpPortScanResult>> for BacklogTcpPortScanRequest {
        fn from(value: Vec<TcpPortScanResult>) -> Self {
            let mut entries: Vec<BacklogTcpPortScanResult> = Vec::new();
            entries.reserve(value.len());

            for e in value {
                entries.push(e.into());
            }
            Self { entries }
        }
    }

    impl From<HostAliveResult> for BacklogHostAliveResult {
        fn from(value: HostAliveResult) -> Self {
            let address = match value.host {
                IpNetwork::V4(v) => Address {
                    address: Some(shared::address::Address::Ipv4((v.ip()).into())),
                },
                IpNetwork::V6(v) => Address {
                    address: Some(shared::address::Address::Ipv6((v.ip()).into())),
                },
            };
            Self {
                attack_uuid: value.attack.to_string(),
                host: Some(address),
            }
        }
    }

    impl From<Vec<HostAliveResult>> for BacklogHostAliveRequest {
        fn from(value: Vec<HostAliveResult>) -> Self {
            let mut entries: Vec<BacklogHostAliveResult> = Vec::new();
            entries.reserve(value.len());

            for e in value {
                entries.push(e.into());
            }
            Self { entries }
        }
    }
}

/// Starts the gRPC server
///
/// **Parameter**:
/// - `config`: Reference to [Config]
pub async fn start_rpc_server(config: &Config, backlog: Backlog) -> Result<(), Box<dyn Error>> {
    info!("Starting Server");
    Server::builder()
        .add_service(ReqAttackServiceServer::new(Attacks { backlog }))
        .serve(SocketAddr::new(
            config.server.listen_address.parse().unwrap(),
            config.server.listen_port,
        ))
        .await?;

    Ok(())
}
