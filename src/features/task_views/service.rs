use super::dto::{TaskViewDbDto, UpdateTaskViewDto, CreateSharedTaskViewDto};
use crate::cache::{set_redis_cache, delete_redis_cache, get_redis_cache};
use super::model::{TaskViewPermission, SharedTaskView};
use crate::errors::{ServiceResult, ServiceError};
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Fetch the task view permission for the given user and view ID (Cache first, then database)
async fn get_task_view_permission(state: &AppState, user_id: &Uuid, view_id: i64) -> ServiceResult<TaskViewPermission> {
    let cache_key = format!("tvp:{}:{}", view_id, user_id); // TVP = Task View Permission

    // Check if the Redis has the task view permission cached
    let cached_permission: Option<TaskViewPermission> = get_redis_cache(&state.redis_cache, &cache_key).await?;
    if let Some(permission) = cached_permission {
        // Increase the TTL by resetting the cache with the same value and TTL
        set_redis_cache(&state.redis_cache, &cache_key, &permission, 1 * 60 * 60).await?;

        return Ok(permission);
    }

    // If not in cache, fetch from the database
    let permission = repository::check_task_view_ownership(&state.pg_pool, user_id, view_id).await?;

    // Cache the permission in Redis for future requests
    if let Some(ref perm) = permission {
        set_redis_cache(&state.redis_cache, &cache_key, perm, 1 * 60 * 60).await?;
    }

    // If the permission is none, return a forbidden error indicating lack of access or non-existence of the task view
    if permission.is_none() {
        return Err(ServiceError::Forbidden("You do not have permission to access this task view or it does not exist".into()));
    }

    Ok(permission.unwrap())
}


/// Fetch the task view for the given user and view ID, ensuring the user has the necessary permissions.
pub async fn get_task_view_by_user(state: &AppState, user_id: &Uuid, view_id: i64) -> ServiceResult<TaskViewDbDto> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, user_id, view_id).await?;

    // List task views for the session user
    let view = repository::get_task_view(&state.pg_pool, &task_view_permission.owner_id, view_id).await?;
    if view.is_none() {
        return Err(ServiceError::NotFound("Task view not found".into()));
    }

    Ok(view.unwrap())
}


/// Update an existing task view, self only possible
pub async fn update_task_view(state: &AppState, user_id: &Uuid, new_info: &UpdateTaskViewDto) -> ServiceResult<()> {
    // Update the task view in the database
    let result = repository::update_task_view(&state.pg_pool, user_id, new_info).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Task view not found or no changes made".into()));
    }

    // Invalidate the cache for all task view permissions related to this view ID
    delete_redis_cache(&state.redis_cache, &format!("tvp:{}:*", new_info.view_id)).await?;

    Ok(())
}


/// Delete an existing task view, self only possible
pub async fn delete_task_view(state: &AppState, user_id: &Uuid, view_id: i64) -> ServiceResult<()> {
    // Delete the task view in the database
    let result = repository::delete_task_view(&state.pg_pool, user_id, view_id).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Task view not found or no changes made".into()));
    }

    // Invalidate the cache for all task view permissions related to this view ID
    delete_redis_cache(&state.redis_cache, &format!("tvp:{}:*", view_id)).await?;

    Ok(())
}


/// Delete an existing shared task view, self only possible (Shared with others remove them from the view)
pub async fn remove_user_from_shared_task_view(state: &AppState, session_user_id: &Uuid, shared_view_id: i64, user_id: &Uuid) -> ServiceResult<()> {
    // Make sure the session user is the owner of the shared task view
    let shared_view_permission = get_task_view_permission(&state, session_user_id, shared_view_id).await?;
    if !shared_view_permission.is_owner {
        return Err(ServiceError::Forbidden("Only the owner can remove users from the shared task view".into()));
    }

    // Delete the shared task view in the database
    let result = repository::delete_shared_task_view(&state.pg_pool, session_user_id, shared_view_id, user_id).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Shared task view not found or no changes made".into()));
    }

    // Invalidate the cache for all task view permissions related to this view ID
    delete_redis_cache(&state.redis_cache, &format!("tvp:{}:{}", shared_view_id, user_id)).await?;

    Ok(())
}


/// Update an existing shared task view, self only possible (Only the owner can update an share)
pub async fn update_shared_task_view(state: &AppState, session_user_id: &Uuid, new_info: &CreateSharedTaskViewDto) -> ServiceResult<u64> {
    // Make sure the session user is the owner of the shared task view
    let shared_view_permission = get_task_view_permission(&state, session_user_id, new_info.view_id).await?;
    if !shared_view_permission.is_owner {
        return Err(ServiceError::Forbidden("Only the owner can update the shared task view".into()));
    }

    // Update the shared task view in the database
    let result = repository::update_shared_task_view(&state.pg_pool, session_user_id, new_info).await?;

    // Invalidate the cache for all task view permissions related to this view ID
    delete_redis_cache(&state.redis_cache, &format!("tvp:{}:*", new_info.view_id)).await?;

    Ok(result)
}


/// List all shared users for a given task view, only the owner can perform this action
pub async fn list_all_shared_users(state: &AppState, session_user_id: &Uuid, view_id: i64) -> ServiceResult<Vec<SharedTaskView>> {
    // Make sure the session user is the owner of the shared task view
    let shared_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;
    if !shared_view_permission.is_owner {
        return Err(ServiceError::Forbidden("You do not have permission to view the shared users for this task view".into()));
    }

    // Since the session user is the owner, they have access to all shared users for this task view
    let shared_users = repository::list_all_shared_view_users(&state.pg_pool, view_id).await?;

    Ok(shared_users)    
}


/// Get a shared task view by its ID, either as the owner or as a shared user
pub async fn get_shared_task_view_user_info(state: &AppState, session_user_id: &Uuid, view_id: i64, user_id: &Uuid) -> ServiceResult<Option<SharedTaskView>> {
    // Get the task view permission for the session user
    let shared_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // If the session user is the owner, they can access any shared user information associated with this view ID
    if shared_view_permission.is_owner {
        let info = repository::get_one_shared_view_user_info(&state.pg_pool, view_id, user_id).await?;

        return Ok(info);
    }

    // If the session user is not the owner, they can only access their own shared task view
    if *session_user_id != *user_id {
        return Err(ServiceError::Forbidden("You do not have permission to view this shared task view user".into()));
    }

    // Since the session user is not the owner but is the same as the requested user, fetch their shared task view
    let info = repository::get_one_shared_view_user_info(&state.pg_pool, view_id, user_id).await?;

    Ok(info)
}
