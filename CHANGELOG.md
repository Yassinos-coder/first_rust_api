# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-05

### Added
- Axum web server with typed environment configuration (port, API prefix, database URL)
- Postgres connection pool shared through application state
- Health check endpoint
- User registration endpoint with Argon2 password hashing and input validation
- Password update endpoint for existing users
- Consistent JSON error responses for bad requests, missing users, conflicts and internal errors
- Users table migration
- Example environment file and a `.gitignore` that keeps secrets and local files out of the repository
