use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "offer.jresearch.ai",
    version = "v1",
    kind = "OfferDeployment",
    plural = "offerdeployments",
    namespaced
)]
pub struct OfferDeploymentSpec {
    pub app: String,
}
