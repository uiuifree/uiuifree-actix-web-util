use crate::error::CustomError;
use mysql::prelude::{AsStatement, Queryable};
pub use mysql::Pool;
use mysql::{Opts, PooledConn};
use crate::env::get_env;

fn mysql_url() -> String {
    get_env("DATABASE_URL", "")
}


pub fn mysql_search<T, S, P>(pool: &Pool, stmt: S, params: P) -> Result<Vec<T>, CustomError>
where
    S: AsStatement,
    P: Into<mysql::Params>,
    T: mysql::prelude::FromRow,
    mysql::Params: std::convert::From<P>,
{
    let mut conn = mysql_pool_to_connection(pool)?;
    let res = conn.exec(stmt, params);

    if res.is_err() {
        return Err(CustomError::Database("SQL ERROR".to_string()));
    }
    Ok(res.unwrap_or_default())
}
pub fn mysql_search_first<T, S, P>(
    pool: &Pool,
    stmt: S,
    params: P,
) -> Result<Option<T>, CustomError>
where
    S: AsStatement,
    P: Into<mysql::Params>,
    T: mysql::prelude::FromRow,
    mysql::Params: std::convert::From<P>,
{
    let mut conn = mysql_pool_to_connection(pool)?;
    let res = conn.exec_first(stmt, params);
    if res.is_err() {
        return Err(CustomError::Database("SQL ERROR".to_string()));
    }
    Ok(res.unwrap())
}
pub fn mysql_update<S, P>(pool: &Pool, stmt: S, params: P) -> Result<u64, CustomError>
where
    S: AsStatement,
    P: Into<mysql::Params>,
    mysql::Params: std::convert::From<P>,
{
    let mut conn = mysql_pool_to_connection(pool)?;
    let res = conn.exec_drop(stmt, params);
    if res.is_err() {
        return Err(CustomError::Database("SQL ERROR".to_string()));
    }
    Ok(conn.affected_rows())
}

pub fn mysql_pool_to_connection(pool: &Pool) -> Result<PooledConn, CustomError> {
    let conn = pool.get_conn();
    if let Err(..) = conn {
        return Err(CustomError::Database(conn.unwrap_err().to_string()));
    }
    Ok(conn.unwrap())
}

pub fn get_mysql_pool() -> Result<Pool, CustomError> {
    let url = mysql_url();
    let opts = Opts::from_url(url.as_str());
    if opts.is_err() {
        return Err(CustomError::Database(opts.err().unwrap().to_string()));
    }
    // let pool = Pool::new(opts.unwrap());
    let pool = Pool::new_manual(1, 20, opts.unwrap());
    if pool.is_err() {
        return Err(CustomError::Database(pool.err().unwrap().to_string()));
    }
    Ok(pool.unwrap())
}
pub fn get_mysql_pool_manual(min: usize, max: usize) -> Result<Pool, CustomError> {
    let url = mysql_url();
    let opts = Opts::from_url(url.as_str());
    if opts.is_err() {
        return Err(CustomError::Database(opts.err().unwrap().to_string()));
    }
    // let pool = Pool::new(opts.unwrap());
    let opts = opts.unwrap();
    let pool = Pool::new_manual(min, max, opts);
    if pool.is_err() {
        return Err(CustomError::Database(pool.err().unwrap().to_string()));
    }
    Ok(pool.unwrap())
}