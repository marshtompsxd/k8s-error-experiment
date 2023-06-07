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

    // The update succeeds
    match cm_api
        .replace(
            &cm_name,
            &pp,
            &ConfigMap {
                metadata: ObjectMeta {
                    resource_version: None,
                    ..created_cm.metadata.clone()
                },
                ..created_cm.clone()
            },
        )
        .await
    {
        Err(e) => {
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    println!(
        "new rv is {}",
        cm_api
            .get(&cm_name)
            .await
            .unwrap()
            .metadata
            .resource_version
            .as_ref()
            .unwrap()
    );

    Ok(())
}
