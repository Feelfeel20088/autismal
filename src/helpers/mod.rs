use std::ops::RangeInclusive;

use tokio::{fs::File, io::AsyncReadExt};

/// Generate a random value within the given inclusive range.
///
/// Example:
/// ```
/// let num = generate_async_safe_random(1..=10);
/// println!("Random number: {}", num);
/// ```
pub async fn generate_thread_safe_random<T>(range: RangeInclusive<T>) -> Result<T, std::io::Error>
where
    T: Copy + Into<u128> + TryFrom<u128> + std::fmt::Debug,
{
    let mut file = File::open("/dev/urandom").await?;

    let mut buf = [0u8; 16];

    file.read_exact(&mut buf).await?;

    let raw = u128::from_ne_bytes(buf);

    let min: u128 = (*range.start()).into();

    let max: u128 = (*range.end()).into();

    let span = max - min + 1;

    let scaled = min + (raw % span);

    let result = T::try_from(scaled).ok().unwrap();

    Ok(result)
}
