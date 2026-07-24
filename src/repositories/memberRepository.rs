use crate::{enums::user_role::Role, models::project::Project};
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::models::member::Member;


pub async fn create_member(pool: &MySqlPool, user_id: Uuid, project_id: Uuid, role: Role)->Result<Member, sqlx::Error>{
    let membership = Member{
        user_id,
        project_id,
        role,
        joined_at: chrono::Utc::now()
    };

    sqlx::query("INSERT INTO Members (user_id, project_id, role, joined_at)
                    VALUES (?,?,?,?);")
                    .bind(user_id)
                    .bind(project_id)
                    .bind(role)
                    .bind(membership.joined_at)
                    .execute(pool)
                    .await?;


    Ok(membership)
}



pub async fn remove_member(pool: &MySqlPool, user_id: Uuid, project_id: Uuid)->Result<(),sqlx::Error>{
    sqlx::query("DELETE FROM Members
                    WHERE user_id = ? AND project_id = ?;")
                .bind(user_id)
                .bind(project_id)
                .execute(pool)
                .await?;

    Ok(())

}


pub async fn update_role(pool: &MySqlPool, user_id: Uuid, project_id: Uuid, new_role: Role)->Result<(),sqlx::Error>{
    sqlx::query("UPDATE Members
                SET role = ?
                WHERE user_id = ? AND project_id = ?;")
                .bind(new_role)
                .bind(user_id)
                .bind(project_id)
                .execute(pool)
                .await?;
            Ok(())
    
}


pub async fn all_project_members(pool: &MySqlPool, project_id:Uuid)->Result<Vec<Member>, sqlx::Error>{
    let members = sqlx::query_as::<_,Member>(
        "SELECT * FROM Member
              WHERE project_id = ?;")
            .bind(project_id)
            .fetch_all(pool)
            .await?;


    Ok(members)
}