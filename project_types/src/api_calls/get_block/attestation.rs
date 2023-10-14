use serde_derive::{Serialize, Deserialize};

use super::{attestation_data::AttestationData, Signature};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub aggregation_bits: String,
    pub data: AttestationData,
    pub signature: Signature,
}
