use thirtyfour::prelude::*;

async fn create_driver() -> WebDriver {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_debugger_address("127.0.0.1:9000").unwrap();
    let driver = WebDriver::new("http://127.0.0.1:9515", caps).await.unwrap();
    driver.goto("http://127.0.0.1:8080").await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs_f32(0.5)).await;
    driver
}

#[tokio::test]
async fn test_simple() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let button = driver.find(By::Id("BUTTON")).await?;

    assert!(driver.find(By::Id("SIMPLE")).await.is_err());
    button.click().await?;
    assert!(driver.find(By::Id("SIMPLE")).await.is_ok());
    button.click().await?;
    assert!(driver.find(By::Id("SIMPLE")).await.is_err());

    Ok(())
}

#[tokio::test]
async fn test_reactive() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let button = driver.find(By::Id("BUTTON")).await?;

    button.click().await?;
    let element = driver.find(By::Id("REACTIVE")).await?;
    assert_eq!(element.text().await?, "true");

    Ok(())
}

#[tokio::test]
async fn test_sub() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let button = driver.find(By::Id("BUTTON")).await?;

    button.click().await?;
    let element = driver.find(By::Id("SUB")).await?;
    assert_eq!(element.text().await?, "SUB");

    Ok(())
}
