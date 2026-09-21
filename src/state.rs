use aws_sdk_s3::Client as R2Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub r2: R2Client,
    pub r2_bucket: String,
}
