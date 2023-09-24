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
#[ignore]
async fn test_simple() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let button = driver.find(By::Id("BUTTON")).await?;

    assert!(driver.find(By::Id("SIMPLE")).await.is_err());
    button.click().await?;
    assert!(driver.find(By::Id("SIMPLE")).await.is_ok());

    Ok(())
}

