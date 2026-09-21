use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

pub async fn build_r2_client() -> Client {
    let account_id = std::env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID should be set.");
    let access_key_id = std::env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID should be set.");
    let secret_access_key =
        std::env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY should be set.");

    let credentials = Credentials::new(access_key_id, secret_access_key, None, None, "R2");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(format!("https://{account_id}.r2.cloudflarestorage.com"))
        .credentials_provider(credentials)
        .region(Region::new("auto"))
        .load()
        .await;

    Client::new(&config)
}
