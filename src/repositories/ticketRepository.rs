use crate::models::ticket::Ticket;
use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::enums::ticket_status::TicketStatus;
use crate::enums::ticket_priority::TicketPriority;

pub async fn create_ticket(
    pool: &MySqlPool,
    id: Uuid, 
    project_id: Uuid, 
    sprint_id: Uuid, 
    title: &str, 
    description: &str,
    creator_id: Uuid,
    assignee_id: Option<Uuid>,
    status: TicketStatus,
    priority: TicketPriority,
)->Result<Ticket, sqlx::Error>{
    let ticket = Ticket{
        id,
        project_id,
        sprint_id: Some(sprint_id),
        title: title.to_string(),
        description: Some(description.to_string()),
        creator_id,
        assignee_id,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        status,
        priority
    };


    sqlx::query("INSERT INTO Tickets (id, project_id, sprint_id, title, description, creator_id, assignee_id, created_at, updated_at, status, priority)
                    VALUES (?,?,?,?,?,?,?,?,?,?,?)")
                .bind(id)
                .bind(project_id)
                .bind(sprint_id)
                .bind(title)
                .bind(description)
                .bind(creator_id)
                .bind(assignee_id)
                .bind(ticket.created_at)
                .bind(ticket.updated_at)
                .bind(status)
                .bind(priority)
                .execute(pool)
                .await?;

    Ok(ticket)
}


pub async fn get_ticket_by_id(pool: &MySqlPool, id: Uuid)->Result<Ticket, sqlx::Error>{
    let ticket = sqlx::query_as::<_, Ticket>("SELECT * FROM Tickets WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(ticket)
}


pub async fn get_tickets_by_project_id(pool: &MySqlPool, project_id: Uuid)->Result<Vec<Ticket>, sqlx::Error>{
    let tickets = sqlx::query_as::<_, Ticket>("SELECT * FROM Tickets WHERE project_id = ?")
        .bind(project_id) 
        .fetch_all(pool)
        .await?;

    Ok(tickets)
}


pub async fn get_tickets_by_sprint_id(pool: &MySqlPool, sprint_id: Uuid)->Result<Vec<Ticket>, sqlx::Error>{
    let tickets = sqlx::query_as::<_, Ticket>("SELECT * FROM Tickets WHERE sprint_id = ?")
        .bind(sprint_id)
        .fetch_all(pool)
        .await?;

    Ok(tickets)
}

pub async fn update_ticket(pool: &MySqlPool, id: Uuid, title: &str, description: &str, priority: TicketPriority)->Result<(), sqlx::Error>{

    sqlx::query("UPDATE Tickets SET title = ?, description = ?, priority = ? WHERE id = ?")
        .bind(title)
        .bind(description)
        .bind(priority)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn update_ticket_status(pool: &MySqlPool, id: Uuid, status: TicketStatus)->Result<(), sqlx::Error>{
    sqlx::query("UPDATE Tickets SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_ticket_assignee(pool: &MySqlPool, id: Uuid, assignee_id: Uuid)->Result<(), sqlx::Error>{
    sqlx::query("UPDATE Tickets SET assignee_id = ? WHERE id = ?")
        .bind(assignee_id)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn set_ticket_assignee_to_null(pool: &MySqlPool, id: Uuid)->Result<(), sqlx::Error>{
    sqlx::query("UPDATE Tickets SET assignee_id = NULL WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn update_ticket_sprint(pool: &MySqlPool, id: Uuid, sprint_id: Uuid)->Result<(), sqlx::Error>{
    sqlx::query("UPDATE Tickets SET sprint_id = ? WHERE id = ?")
        .bind(sprint_id)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn set_ticket_sprint_to_null(pool: &MySqlPool, id: Uuid)->Result<(), sqlx::Error>{
    sqlx::query("UPDATE Tickets SET sprint_id = NULL WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())

}

pub async fn delete_ticket(pool: &MySqlPool, id: Uuid)->Result<(), sqlx::Error>{
    sqlx::query("DELETE FROM Tickets WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}