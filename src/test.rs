#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use snowflake::SnowflakeIdGenerator;

    #[test]
    fn snowflake() {
        let mut id_gen = SnowflakeIdGenerator::new(1, 1);
        for _i in 0..10 {
            let id = id_gen.real_time_generate();
            println!("{id}");
            sleep(Duration::from_secs(2));
        }
    }
}