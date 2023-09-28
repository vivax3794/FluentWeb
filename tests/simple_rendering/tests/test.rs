use thirtyfour::prelude::*;

async fn create_driver() -> WebDriver {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("127.0.0.1:9000").unwrap();
    let driver = WebDriver::new("http://127.0.0.1:9515", caps).await.unwrap();
    driver.goto("http://127.0.0.1:8080").await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    driver
}

#[tokio::test]
async fn test_simple_render() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let body = driver.find(By::Tag("body")).await?;
    let text = body.text().await?;
    assert!(text.contains("CONTAIN"));

    Ok(())
}

#[tokio::test]
async fn test_expression() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let body = driver.find(By::Tag("body")).await?;
    let text = body.text().await?;
    assert!(text.contains('4'));

    Ok(())
}
