use std::collections::HashMap;

use sqlx::{Sqlite, SqliteConnection, SqlitePool};

use crate::engine::{
    checks::{check::CheckFieldValue, configure_check},
    service::Service,
};

pub async fn get_current_round_number(db: SqlitePool) -> Option<i64> {
    sqlx::query!(r#"SELECT MAX(number) as "current_round_number?: i64" FROM rounds"#)
        .fetch_one(&db)
        .await
        .unwrap()
        .current_round_number
}

pub async fn get_service_checks(db: SqlitePool) {
    // let checks = sqlx::query!("").fetch_all(db).await.unwrap();
    //
    // for round in rounds {
    //     round.ro
    // }
}

pub async fn is_service_name_available(db: SqlitePool) -> bool {
    sqlx::query!(r#"SELECT COUNT(*) as "number_of_users" FROM users"#)
        .fetch_one(&db)
        .await
        .unwrap()
        .number_of_users
        > 0
}

pub async fn create_team(db: SqlitePool, team_name: &str) {
    sqlx::query!("INSERT INTO teams (name) VALUES ($1)", team_name)
        .execute(&db)
        .await
        .unwrap();
}

pub async fn create_service(
    db: SqlitePool,
    service_name: &str,
    weight: i64,
    check_name: &str,
    check_configuration: HashMap<String, CheckFieldValue>,
) {
    let config_json = serde_json::to_string(&check_configuration).unwrap();

    sqlx::query!(
        "INSERT INTO services (name, weight, check_name, check_configuration) VALUES ($1, $2, $3, $4)",
        service_name,
        weight,
        check_name,
        config_json,
    )
    .execute(&db)
    .await
    .unwrap();
}

pub async fn does_service_exist(db: SqlitePool, service_name: &str) -> bool {
    sqlx::query!(
        r#"SELECT COUNT(*) as "number_of_services" FROM services WHERE name == $1"#,
        service_name
    )
    .fetch_one(&db)
    .await
    .unwrap()
    .number_of_services
        > 0
}

pub async fn get_services(db: SqlitePool) -> Vec<Service> {
    let services =
        sqlx::query!("SELECT name, weight, check_name, check_configuration FROM services")
            .fetch_all(&db)
            .await
            .unwrap();

    let mut constructed_services = vec![];

    for service in services {
        let check_config: HashMap<String, CheckFieldValue> =
            serde_json::from_str(&service.check_configuration).unwrap();

        constructed_services.push(Service {
            name: service.name,
            weight: service.weight,
            check: configure_check(&service.check_name, check_config).unwrap(),
        });
    }

    constructed_services
}

pub async fn delete_service(db: SqlitePool, service_name: &str) {
    sqlx::query!("DELETE FROM services WHERE name == $1", service_name)
        .execute(&db)
        .await
        .unwrap();
}
