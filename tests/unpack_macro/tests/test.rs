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
async fn updates_dom() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let button = driver.find(By::Id("BUTTON")).await?;
    let content = driver.find(By::Id("CONTENT")).await?;

    assert_eq!(content.text().await?, "0");
    button.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert_eq!(content.text().await?, "10");

    Ok(())
}
