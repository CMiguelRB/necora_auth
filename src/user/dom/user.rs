pub struct User {
    id: String,
    username: String,
    password: String,
    refreshToken: String,
    isActive: bool,
    createdAt: std::time::Instant,
    updatedAt: std::time::Instant,
    lastAccess: std::time::Instant
}