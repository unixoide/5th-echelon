//! Implements client-side logic for interacting with the server's gRPC API
//! for user authentication.
//!
//! This module provides functions for testing user logins and registering new users.

use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use server_api::users::RegisterRequest;

use super::Error;

/// Tests a user login against the API server.
///
/// # Arguments
///
/// * `api_url` - The URL of the API server.
/// * `username` - The username to test.
/// * `password` - The password to test.
///
/// # Returns
///
/// An `Ok(())` if the login is successful, or an `Error` otherwise.
pub async fn test_login(api_url: String, username: &str, password: &str) -> Result<(), Error> {
    let Ok(mut client) = UsersClient::connect(api_url).await else {
        return Err(Error::ConnectionFailed);
    };

    let resp = match client
        .login(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            // Handle different gRPC status codes.
            if matches!(status.code(), tonic::Code::Unauthenticated) {
                return Err(Error::InvalidPassword);
            }
            if matches!(status.code(), tonic::Code::NotFound) {
                return Err(Error::UserNotFound);
            } else {
                return Err(Error::SendingRequestFailed);
            }
        }
    };

    let resp = resp.into_inner();
    if resp.error.is_empty() {
        Ok(())
    } else {
        Err(Error::ServerFailure(resp.error))
    }
}

/// Registers a new user with the API server.
///
/// # Arguments
///
/// * `api_url` - The URL of the API server.
/// * `username` - The desired username.
/// * `password` - The desired password.
/// * `ubi_id` - The user's Ubisoft ID.
///
/// # Returns
///
/// An `Ok(())` if the registration is successful, or an `Error` otherwise.
pub async fn register(api_url: String, username: &str, password: &str, ubi_id: &str) -> Result<(), Error> {
    let Ok(mut client) = UsersClient::connect(api_url).await else {
        return Err(Error::ConnectionFailed);
    };

    let resp = match client
        .register(RegisterRequest {
            username: username.to_string(),
            password: password.to_string(),
            ubi_id: ubi_id.to_string(),
        })
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            // Handle different gRPC status codes.
            if matches!(status.code(), tonic::Code::AlreadyExists) {
                return Err(Error::UsernameAlreadyTaken);
            } else {
                return Err(Error::SendingRequestFailed);
            }
        }
    };

    let resp = resp.into_inner();
    if resp.error.is_empty() {
        Ok(())
    } else {
        Err(Error::ServerFailure(resp.error))
    }
}
