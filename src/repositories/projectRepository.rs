use crate::models::project::Project;
use sqlx::MySqlPool;
use uuid::Uuid;



pub async fn create_project(
    pool: &MySqlPool,
    title: &str,
    description: &str,
    owner_id: Uuid,
) -> Result<Project, sqlx::Error> {
    let project = Project {
        id: Uuid::new_v4(),
        title: title.to_string(),
        description: Some(description.to_string()),
        owner_id,
        creation_date: chrono::Utc::now(),
    };

    sqlx::query(
        "INSERT INTO projects (id, title, description, owner_id, creation_date)
         VALUES (?, ?, ?, ?, ?);",
    )
    .bind(project.id)
    .bind(&project.title)
    .bind(&project.description)
    .bind(project.owner_id)
    .bind(project.creation_date)
    .execute(pool)
    .await?;

    Ok(project)
}
pub async fn get_project_by_id(pool : &MySqlPool, id: Uuid)->Result<Project, sqlx::Error>{
    let sql = "SELECT * FROM projects WHERE id = ?;";
    let result = sqlx::query_as::<_, Project>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    let Some(project) = result else {
        return Err(sqlx::Error::RowNotFound);
    };
    Ok(project)
}


pub async fn get_projects_by_owner_id(pool : &MySqlPool, owner_id: Uuid)->Result<Vec<Project>, sqlx::Error>{
    let sql = "SELECT * FROM projects WHERE owner_id = ?;";
    let result = sqlx::query_as::<_, Project>(sql)
        .bind(owner_id)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn update_project(pool : &MySqlPool, id: Uuid, title: &str, description: &str)->Result<(), sqlx::Error>{
    let sql = "UPDATE projects
                    SET title = ?, description = ? 
                    WHERE id = ?;";
    let result = sqlx::query(sql)
        .bind(title)
        .bind(description)
        .bind(id)
        .execute(pool)
        .await?;
   
    Ok(())
}

pub async fn delete_project(pool : &MySqlPool, id: Uuid)->Result<(), sqlx::Error>{
    let sql = "DELETE FROM projects WHERE id = ?;";
    let result = sqlx::query(sql)
        .bind(id)
        .execute(pool)
        .await?;
   
    Ok(())
}