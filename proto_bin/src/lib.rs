#[cfg(feature = "user_api")]
pub mod user_api {
    tonic::include_proto!("user_api");
}

#[cfg(feature = "order_api")]
pub mod order_api{
    tonic::include_proto!("order_api");
}

#[cfg(feature = "auth_api")]
pub mod auth_api{
    tonic::include_proto!("auth_api");
}