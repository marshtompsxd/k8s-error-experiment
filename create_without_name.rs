// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), "default");
    let cm = ConfigMap {
        metadata: ObjectMeta::default(),
        data: None,
        ..Default::default()
    };
    let pp = PostParams::default();
    match cm_api.create(&pp, &cm).await {
        Err(e) => {
            // You are expected to see the error:
            // ApiError: ConfigMap "" is invalid: metadata.name: Required value: name or generateName is required: Invalid (ErrorResponse { status: "Failure", message: "ConfigMap \"\" is invalid: metadata.name: Required value: name or generateName is required", reason: "Invalid", code: 422 })
            println!("This creation fails with:\n{}", e);
        }
        _ => {}
    }

    Ok(())
}
