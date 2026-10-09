use crate::cache::{set_redis_cache, delete_redis_cache, get_redis_cache};
use super::dto::{TaskViewDbDto, UpdateTaskViewDto};
use crate::errors::{ServiceResult, ServiceError};
use super::model::TaskViewPermission;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Fetch the task view permission for the given user and view ID (Cache first, then database)
async fn get_task_view_permission(state: &AppState, user_id: &Uuid, view_id: i64) -> ServiceResult<TaskViewPermission> {
    let cache_key = format!("tvp:{}:{}", view_id, user_id);

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
