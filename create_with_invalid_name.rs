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
        metadata: ObjectMeta {
            name: Some("my_configmap".to_string()), // valid name should not contain "_"
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };
    let pp = PostParams::default();
    match cm_api.create(&pp, &cm).await {
        Err(e) => {
            // You are expected to see the error:
            // ApiError: ConfigMap "my_configmap" is invalid: metadata.name: Invalid value: "my_configmap": a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*'): Invalid (ErrorResponse { status: "Failure", message: "ConfigMap \"my_configmap\" is invalid: metadata.name: Invalid value: \"my_configmap\": a lowercase RFC 1123 subdomain must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character (e.g. 'example.com', regex used for validation is '[a-z0-9]([-a-z0-9]*[a-z0-9])?(\\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*')", reason: "Invalid", code: 422 })
            println!("This creation fails with:\n{}", e);
        }
        _ => {}
    }

    Ok(())
}
