macro_rules! unwrap {
    ($result:expr) => {{
        use rocket::response::status;
        $result.map_err(|_| status::BadRequest(None))?
    }};

    ($result:expr, $msg:expr) => {{
        use rocket::response::status;
        $result.map_err(|_| status::BadRequest(Some($msg.into())))?
    }}
}

macro_rules! debug {
    ($($args:tt)*) => {{
        if cfg!(debug) {
            println!($($args)*)
        }
    }}
}
