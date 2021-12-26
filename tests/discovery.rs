use std::time::Duration;

use rues::Bridge;

#[tokio::test]
pub async fn discover() {
	drop(Bridge::discover(Duration::from_secs(3)).await);
}
