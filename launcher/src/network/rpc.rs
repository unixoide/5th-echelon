use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use server_api::users::RegisterRequest;

use super::Error;

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
