use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use rorm::prelude::*;
use uuid::Uuid;

use crate::models::{
    Attack, CertificateTransparencyResult, CertificateTransparencyValueName, DehashedQueryResult,
    DnsRecordResult, DnsRecordType, DnsTxtScanAttackResult, DnsTxtScanServiceHintEntry,
    DnsTxtScanServiceHintType, DnsTxtScanSpfEntry, DnsTxtScanSpfType, DnsTxtScanSummaryType,
    HostAliveResult, ServiceCertainty, ServiceDetectionResult, UdpServiceDetectionResult,
};

pub(crate) type BruteforceSubdomainsResultInsert = DnsRecordResultInsert;
pub(crate) type DnsResolutionResultInsert = DnsRecordResultInsert;

#[derive(Patch)]
#[rorm(model = "DnsRecordResult")]
pub(crate) struct DnsRecordResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) source: String,
    pub(crate) destination: String,
    pub(crate) dns_record_type: DnsRecordType,
}

#[derive(Patch)]
#[rorm(model = "DnsTxtScanAttackResult")]
pub(crate) struct DnsTxtScanAttackResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) domain: String,
    pub(crate) collection_type: DnsTxtScanSummaryType,
}

#[derive(Patch)]
#[rorm(model = "DnsTxtScanServiceHintEntry")]
pub(crate) struct DnsTxtScanServiceHintEntryInsert {
    pub(crate) uuid: Uuid,
    pub(crate) collection: ForeignModel<DnsTxtScanAttackResult>,
    pub(crate) rule: String,
    pub(crate) txt_type: DnsTxtScanServiceHintType,
}

#[derive(Patch)]
#[rorm(model = "DnsTxtScanSpfEntry")]
pub(crate) struct DnsTxtScanSpfEntryInsert {
    pub(crate) uuid: Uuid,
    pub(crate) collection: ForeignModel<DnsTxtScanAttackResult>,
    pub(crate) rule: String,
    pub(crate) spf_type: DnsTxtScanSpfType,
    pub(crate) spf_ip: Option<IpNetwork>,
    pub(crate) spf_domain: Option<String>,
    pub(crate) spf_domain_ipv4_cidr: Option<i32>,
    pub(crate) spf_domain_ipv6_cidr: Option<i32>,
}

#[derive(Patch)]
#[rorm(model = "DehashedQueryResult")]
pub(crate) struct DehashedQueryResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) dehashed_id: i64,
    pub(crate) email: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) hashed_password: Option<String>,
    pub(crate) ip_address: Option<IpNetwork>,
    pub(crate) name: Option<String>,
    pub(crate) vin: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) database_name: Option<String>,
}

#[derive(Patch)]
#[rorm(model = "CertificateTransparencyValueName")]
pub(crate) struct CertificateTransparencyValueNameInsert {
    pub(crate) uuid: Uuid,
    pub(crate) value_name: String,
    pub(crate) ct_result: ForeignModel<CertificateTransparencyResult>,
}

#[derive(Patch)]
#[rorm(model = "CertificateTransparencyResult")]
pub(crate) struct CertificateTransparencyResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) issuer_name: String,
    pub(crate) common_name: String,
    pub(crate) not_before: Option<DateTime<Utc>>,
    pub(crate) not_after: Option<DateTime<Utc>>,
    pub(crate) serial_number: String,
}

#[derive(Patch)]
#[rorm(model = "HostAliveResult")]
pub(crate) struct HostAliveResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) host: IpNetwork,
}

#[derive(Patch)]
#[rorm(model = "ServiceDetectionResult")]
pub(crate) struct ServiceDetectionResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) certainty: ServiceCertainty,
    pub(crate) host: IpNetwork,
    pub(crate) port: i32,
}

#[derive(Patch)]
#[rorm(model = "UdpServiceDetectionResult")]
pub(crate) struct UdpServiceDetectionResultInsert {
    pub(crate) uuid: Uuid,
    pub(crate) attack: ForeignModel<Attack>,
    pub(crate) certainty: ServiceCertainty,
    pub(crate) host: IpNetwork,
    pub(crate) port: i32,
}
