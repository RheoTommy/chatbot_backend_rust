use firestore::FirestoreDb;

pub async fn from_env() -> anyhow::Result<FirestoreDb> {
    let project_id = std::env::var("GCP_PROJECT_ID")?;
    Ok(FirestoreDb::new(project_id).await?)
}
