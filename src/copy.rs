use crate::spec::{self, manifest, manifestv1};
use chrono::{DateTime, FixedOffset};
use reqwest;
use serde::{Deserialize, Serialize};

const REGISTRY: &str = "http://localhost:5000/v2/";

// TODO figure out athentication
//const DOCKER_AUTH: &str = "https://auth.docker.io/token?scope=repository";

#[derive(Debug)]
pub struct Copy {
    client: reqwest::Client,
    image: String,
    dest: String,
}

impl Copy {
    pub fn new(image: String, dest: String) -> Copy {
        Copy {
            client: reqwest::Client::new(),
            image,
            dest,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let reply = self.client.get(REGISTRY).send().await?.text().await?;
        println!("first reply {}", reply);
        // TODO make this less stupid
        let image_name: &str = self.image.split(":").collect::<Vec<&str>>()[0];
        let tag: &str = self.image.split(":").collect::<Vec<&str>>()[1];

        // TODO actually check the manifest version
        let reply = self
            .client
            .get(format!("{}/{}/manifests/{}", REGISTRY, image_name, tag))
            .send()
            .await?;
        let content_type = reply.headers().get(reqwest::header::CONTENT_TYPE).unwrap();
        println!("content type {:?}", content_type);

        if content_type.to_str().unwrap().contains(&String::from("v1")) {
            let manifest = reply.json::<manifestv1::Manifest>().await?;
            for layer in manifest.fs_layers {
                println!("{:?}", layer.blob_sum);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthData {
    token: String,
    access_token: String,
    expires_in: u64,
    issued_at: DateTime<FixedOffset>,
}
