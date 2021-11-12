use chrono::{DateTime, FixedOffset};
use reqwest;
use serde::{Deserialize, Serialize};

const REGISTRY: &str = "https://index.docker.io/v2/";
const DOCKER_AUTH: &str = "https://auth.docker.io/token?scope=repository";

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
        let reply: AuthData = self
            .client
            .get(format!(
                "{}:library/{}:pull&service=registry.docker.io",
                DOCKER_AUTH, image_name
            ))
            .send()
            .await?
            .json::<AuthData>()
            .await?;
        println!("auth data {:?}", reply);
        let reply = self
            .client
            .get(format!(
                "{}/library/{}/manifests/{}",
                REGISTRY, image_name, tag
            ))
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", reply.token),
            )
            .send()
            .await?
            .text()
            .await?;
        println!("reply {:?}", reply);

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
