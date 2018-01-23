use std::env;
use std::ops::Deref;

use r2d2::{self, PooledConnection};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref DATABASE_FILE: String = env::var("DATABASE_URL").unwrap();
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(&**DATABASE_FILE);
    Pool::new(manager).expect("db pool")
}

pub struct Postgres(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Postgres {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Postgres {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Postgres, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Postgres(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
