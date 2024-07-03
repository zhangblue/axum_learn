use std::borrow::BorrowMut;

pub fn set_data(redis_connection: &mut redis::Client, key: &str, value: &str) {
    redis::cmd("set").arg(key).arg(value).execute(redis_connection.get_connection().unwrap().borrow_mut())
}

pub fn expire(redis_connection: &mut redis::Client, key: &str, ttl_second: u32) {
    redis::cmd("expire").arg(key).arg(ttl_second).execute(redis_connection.get_connection().unwrap().borrow_mut())
}