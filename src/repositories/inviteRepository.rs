use crate::enums::invitation_status::InvitationStatus;
use crate::models::invitation::Invitation;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn create_invite(
    pool: &MySqlPool,
    project_id: Uuid,
    invited_user_id: Uuid,
    invited_by: Uuid,
) -> Result<Invitation, sqlx::Error> {
    let invitation = Invitation {
        id: Uuid::new_v4(),
        project_id,
        invited_user_id,
        invited_by,
        status: InvitationStatus::Pending,
        created_at: chrono::Utc::now(),
    };

    sqlx::query(
        "INSERT INTO Invitation (id, project_id, invited_user_id, invited_by, status, created_at)
                VALUES (?, ?, ?, ?, ?, ?);",
    )
    .bind(invitation.id)
    .bind(invitation.project_id)
    .bind(invitation.invited_user_id)
    .bind(invitation.invited_by)
    .bind(invitation.status)
    .bind(invitation.created_at)
    .execute(pool)
    .await?;

    Ok(invitation)
}

pub async fn get_invite_by_id(pool: &MySqlPool, id: Uuid) -> Result<Invitation, sqlx::Error> {
    let invite = sqlx::query_as::<_, Invitation>("SELECT * FROM Invitation WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    let Some(invite) = invite else {
        return Err(sqlx::Error::RowNotFound);
    };
    Ok(invite)
}

pub async fn get_invites_by_project_id(
    pool: &MySqlPool,
    project_id: Uuid,
) -> Result<Vec<Invitation>, sqlx::Error> {
    let invites = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM Invitation WHERE project_id = ?",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(invites)
}

pub async fn get_invite_by_user_id(
    pool: &MySqlPool,
    user_id: Uuid,
) -> Result<Vec<Invitation>, sqlx::Error> {
    let invites = sqlx::query_as::<_, Invitation>(
        "SELECT * FROM Invitation WHERE invited_user_id = ?",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(invites)
}

pub async fn update_status(
    pool: &MySqlPool,
    id: Uuid,
    status: InvitationStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Invitation SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_invitation(pool: &MySqlPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Invitation WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
