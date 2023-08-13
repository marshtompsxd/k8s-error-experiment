// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client,
};
use kube_core::{params::DeleteParams, Resource};

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let dp = DeleteParams::default();
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), "default");
    let cm1 = ConfigMap {
        metadata: ObjectMeta {
            name: Some("cm1".to_string()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };
    let cm2 = ConfigMap {
        metadata: ObjectMeta {
            name: Some("cm2".to_string()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };
    cm_api.create(&pp, &cm1).await?;
    cm_api.create(&pp, &cm2).await?;

    let created_cm1 = cm_api.get("cm1").await.unwrap();
    let created_cm2 = cm_api.get("cm2").await.unwrap();

    let cm3 = ConfigMap {
        metadata: ObjectMeta {
            name: Some("cm3".to_string()),
            owner_references: Some(vec![created_cm1.controller_owner_ref(&()).unwrap()]),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    cm_api.create(&pp, &cm3).await?;

    let mut updated_cm3 = cm_api.get("cm3").await.unwrap();

    updated_cm3.metadata.owner_references =
        Some(vec![created_cm2.controller_owner_ref(&()).unwrap()]);

    match cm_api.replace("cm3", &pp, &updated_cm3).await {
        Err(e) => {
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    updated_cm3 = cm_api.get("cm3").await.unwrap();

    updated_cm3.metadata.owner_references = None;

    match cm_api.replace("cm3", &pp, &updated_cm3).await {
        Err(e) => {
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    cm_api.delete("cm1", &dp).await?;
    cm_api.delete("cm2", &dp).await?;

    Ok(())
}
