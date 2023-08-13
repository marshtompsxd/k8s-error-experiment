// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client, CustomResource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[kube(group = "nullable.se", version = "v1", kind = "ConfigMapGenerator")]
#[kube(shortname = "cmg", namespaced)]
struct ConfigMapGeneratorSpec {
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let client = Client::try_default().await?;
    let cmg_api = Api::<ConfigMapGenerator>::namespaced(client.clone(), "default");
    let cmg_name = "my-configmap-generator".to_string();
    let cmg = ConfigMapGenerator {
        metadata: ObjectMeta {
            name: Some(cmg_name.clone()),
            ..ObjectMeta::default()
        },
        spec: ConfigMapGeneratorSpec {
            content: "hello".to_string(),
        },
    };

    cmg_api.create(&pp, &cmg).await?;

    let created_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!(
        "rv is {}",
        created_cmg.metadata.resource_version.as_ref().unwrap()
    );

    // Set the rv to None
    let updated_cmg_1 = ConfigMapGenerator {
        metadata: ObjectMeta {
            resource_version: None,
            ..created_cmg.metadata.clone()
        },
        spec: ConfigMapGeneratorSpec {
            content: "world".to_string(),
        },
    };

    // The update fails with ApiError: configmapgenerators.nullable.se "my-configmap-generator" is invalid: metadata.resourceVersion: Invalid value: 0x0: must be specified for an update: Invalid (ErrorResponse { status: "Failure", message: "configmapgenerators.nullable.se \"my-configmap-generator\" is invalid: metadata.resourceVersion: Invalid value: 0x0: must be specified for an update", reason: "Invalid", code: 422 })
    match cmg_api.replace(&cmg_name, &pp, &updated_cmg_1).await {
        Err(e) => {
            println!("This update fails with:\n{}", e);
        }
        _ => {}
    }

    println!(
        "new rv is {}",
        cmg_api
            .get(&cmg_name)
            .await
            .unwrap()
            .metadata
            .resource_version
            .as_ref()
            .unwrap()
    );

    Ok(())
}
