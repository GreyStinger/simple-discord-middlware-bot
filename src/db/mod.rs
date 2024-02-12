pub mod models;

use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use songbird::typemap::TypeMapKey;

pub struct DatabasePool {
	_conn: ConnectionManager<PgConnection>
}

impl TypeMapKey for DatabasePool {
	type Value = Pool<ConnectionManager<PgConnection>>;
}