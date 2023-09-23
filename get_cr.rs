// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client, CustomResource, CustomResourceExt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[kube(group = "nullable.se", version = "v1", kind = "ConfigMapGenerator")]
#[kube(status = "ConfigMapGeneratorStatus")]
#[kube(shortname = "cmg", namespaced)]
struct ConfigMapGeneratorSpec {
    content: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct ConfigMapGeneratorStatus {
    replicas: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let client = Client::try_default().await?;
    let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());

    let cmgcrd = ConfigMapGenerator::crd();

    crd_api.create(&pp, &cmgcrd).await?;

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
        status: Some(ConfigMapGeneratorStatus { replicas: 3 }),
    };

    cmg_api.create(&pp, &cmg).await?;

    let created_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("{:?}", created_cmg);

    let mut new_cmg = created_cmg;
    new_cmg.status = Some(ConfigMapGeneratorStatus { replicas: 2 });

    cmg_api.replace(&cmg_name, &pp, &new_cmg).await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("{:?}", updated_cmg);

    cmg_api
        .replace_status(&cmg_name, &pp, serde_json::to_vec(&new_cmg)?)
        .await?;
    let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    println!("{:?}", updated_cmg);

    // let mut new_cmg: ConfigMapGenerator = updated_cmg;
    // new_cmg.status = Some(ConfigMapGeneratorStatus { replicas: 2 });

    // let updated_cmg = cmg_api.get(&cmg_name).await.unwrap();
    // println!("{:?}", updated_cmg);

    Ok(())
}
