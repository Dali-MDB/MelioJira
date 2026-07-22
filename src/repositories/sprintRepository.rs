use crate::models::sprint::Sprint;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn create_sprint(
    pool: &MySqlPool,
    name: &str,
    goal: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    created_by: Uuid,
) -> Result<Sprint, sqlx::Error> {
    let sprint = Sprint {
        id: Uuid::new_v4(),
        project_id: project_id,
        created_by: created_by,
        name: name.to_string(),
        goal: Some(goal.to_string()),
        start_date: start_date,
        end_date: end_date,
    };
    sqlx::query(
        "INSERT INTO Sprints (id, project_id, created_by, name, goal, start_date, end_date)
                VALUES (?, ?, ?, ?, ?, ?, ?);",
    )
    .bind(sprint.id)
    .bind(sprint.project_id)
    .bind(sprint.created_by)
    .bind(sprint.name)
    .bind(sprint.goal)
    .bind(sprint.start_date)
    .bind(sprint.end_date)
    .execute(pool)
    .await?;
    Ok(sprint)
}

pub async fn get_sprint_by_id(pool: &MySqlPool, id: Uuid) -> Result<Sprint, sqlx::Error> {
    let result = sqlx::query_as("SELECT * FROM Sprints WHERE id = ?;")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    let Some(sprint) = result else {
        return Err(sqlx::Error::RowNotFound);
    };
    Ok(sprint)
}

pub async fn get_sprints_by_project_id(
    pool: &MySqlPool,
    project_id: Uuid,
) -> Result<Vec<Sprint>, sqlx::Error> {
    let sprints = sqlx::query_as("SELECT * FROM Sprints WHERE project_id = ?;")
        .bind(project_id)
        .fetch_all(pool)
        .await?;
    Ok(sprints)
}

pub async fn update_sprint(
    pool: &MySqlPool,
    id: Uuid,
    name: &str,
    goal: &str,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let sql =
        "UPDATE Sprints SET name = ?, goal = ?, start_date = ?, end_date = ? WHERE id = ?;";
    sqlx::query(sql)
        .bind(name)
        .bind(goal)
        .bind(start_date)
        .bind(end_date)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_sprint(pool: &MySqlPool, id: Uuid) -> Result<(), sqlx::Error> {
    let sql = "DELETE FROM Sprints WHERE id = ?;";
    sqlx::query(sql).bind(id).execute(pool).await?;
    Ok(())
}
