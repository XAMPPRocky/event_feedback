# Proof of concept event feedback backend
This is a proof of concept for a event feedback system.
Requires Rust and postgres.

### Required environment variables

- `DATABASE_URL` - The url to the postgres database.
- `ADMIN_KEY` - Key required to be sent with requests to create events or
  perform destructive actions.
