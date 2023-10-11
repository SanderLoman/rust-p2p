use super::{attestation_data::AttestationData, Signature};

pub struct Attestation {
    pub aggregation_bits: String,
    pub data: AttestationData,
    pub signature: Signature,
}
