// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client,
};
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("running simple-controller");
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), &"default".to_string());
    let cm = ConfigMap {
        metadata: ObjectMeta {
            name: Some("my-configmap".to_string()),
            namespace: Some("not-default".to_string()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };
    let pp = PostParams::default();
    match cm_api.create(&pp, &cm).await {
        Err(e) => {
            // You are expected to see the error:
            // ApiError: the namespace of the provided object does not match the namespace sent on the request: BadRequest (ErrorResponse { status: "Failure", message: "the namespace of the provided object does not match the namespace sent on the request", reason: "BadRequest", code: 400 })
            println!("{}", e);
        }
        _ => {}
    }

    Ok(())
}
