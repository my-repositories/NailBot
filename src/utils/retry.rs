use std::future::Future;
use std::time::Duration;

pub async fn with_retry<F, Fut, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 1;
    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(err) if attempt >= max_attempts => return Err(err),
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(2u64.pow(attempt - 1))).await;
                attempt += 1;
            }
        }
    }
}
