use sqlx::PgPool;
pub async fn get_roles_for_user(pool: &PgPool, user_id: i64) -> sqlx::Result<Vec<String>> {
    sqlx::query_scalar!(
        r#"SELECT r.name FROM roles r JOIN user_roles ur ON ur.role_id = r.id WHERE ur.user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await
}
