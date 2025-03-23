use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{User, CreateUserRequest};

pub struct UserController;

impl UserController {
    // Get all users
    pub async fn get_all_users() -> impl Responder {
        // This would typically fetch from a database
        let users = vec![
            User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
            User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
        ];
        
        HttpResponse::Ok().json(users)
    }
    
    // Get user by ID
    pub async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
        let user_id = path.into_inner();
        
        // In a real app, you would query your database here
        let user = User {
            id: user_id,
            name: "Example User".to_string(),
            email: "user@example.com".to_string(),
        };
        
        HttpResponse::Ok().json(user)
    }
    
    // Create new user
    pub async fn create_user(user_data: web::Json<CreateUserRequest>) -> impl Responder {
        // In a real app, you would save to your database here
        let new_user = User {
            id: 3, // Would be generated or retrieved from DB
            name: user_data.name.clone(),
            email: user_data.email.clone(),
        };
        
        HttpResponse::Created().json(new_user)
    }
    
    // Update user
    pub async fn update_user(
        path: web::Path<u32>,
        user_data: web::Json<CreateUserRequest>
    ) -> impl Responder {
        let user_id = path.into_inner();
        
        // In a real app, you would update your database here
        let updated_user = User {
            id: user_id,
            name: user_data.name.clone(),
            email: user_data.email.clone(),
        };
        
        HttpResponse::Ok().json(updated_user)
    }
    
    // Delete user
    pub async fn delete_user(path: web::Path<u32>) -> impl Responder {
        let user_id = path.into_inner();
        
        // In a real app, you would delete from your database here
        HttpResponse::Ok().json(format!("User with ID {} deleted", user_id))
    }
}