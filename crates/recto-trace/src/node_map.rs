use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::{MarkedContentId, NodeId, OriginPath};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodeMap {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub canonical_to_origin: BTreeMap<NodeId, OriginPath>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub mcid_to_node: BTreeMap<MarkedContentId, NodeId>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub origin_to_canonical: BTreeMap<OriginPath, NodeId>,
}
