// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client,
};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), "default");
    let cm_name = "my-configmap".to_string();
    let cm = ConfigMap {
        metadata: ObjectMeta {
            name: Some(cm_name.clone()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    cm_api.create(&pp, &cm).await?;

    let created_cm = cm_api.get(&cm_name).await.unwrap();
    println!(
        "rv is {}",
        created_cm.metadata.resource_version.as_ref().unwrap()
    );

    let rv_int: i32 = created_cm
        .metadata
        .resource_version
        .as_ref()
        .unwrap()
        .parse()
        .unwrap();
    let incremented_rv_int = rv_int + 1;
    // Set the rv to a fake one (the current one plus one)
    let updated_cm_1 = ConfigMap {
        metadata: ObjectMeta {
            resource_version: Some(incremented_rv_int.to_string()),
            ..created_cm.metadata.clone()
        },
        data: Some(BTreeMap::from([("key".to_string(), "value".to_string())])),
        ..created_cm.clone()
    };

    match cm_api.replace(&cm_name, &pp, &updated_cm_1).await {
        Err(e) => {
            // You are expected to see the error:
            // ApiError: Operation cannot be fulfilled on configmaps "my-configmap": the object has been modified; please apply your changes to the latest version and try again: Conflict (ErrorResponse { status: "Failure", message: "Operation cannot be fulfilled on configmaps \"my-configmap\": the object has been modified; please apply your changes to the latest version and try again", reason: "Conflict", code: 409 })
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    Ok(())
}
