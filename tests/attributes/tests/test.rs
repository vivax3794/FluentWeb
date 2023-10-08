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
async fn test_conditional() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let checkbox = driver.find(By::Id("CHECK")).await?;

    let element = driver.find(By::Id("A")).await?;

    assert!(element.attr("hidden").await?.is_none());
    checkbox.click().await?;
    assert!(element.attr("hidden").await?.is_some());
    checkbox.click().await?;
    assert!(element.attr("hidden").await?.is_none());

    Ok(())
}

#[tokio::test]
async fn test_computed() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let checkbox = driver.find(By::Id("CHECK")).await?;

    let element = driver.find(By::Id("B")).await?;

    assert_eq!(element.attr("val").await?, Some("false".into()));
    checkbox.click().await?;
    assert_eq!(element.attr("val").await?, Some("true".into()));
    checkbox.click().await?;
    assert_eq!(element.attr("val").await?, Some("false".into()));

    Ok(())
}

#[tokio::test]
async fn test_props() -> WebDriverResult<()> {
    let driver = create_driver().await;
    let checkbox = driver.find(By::Id("CHECK")).await?;

    let element = driver.find(By::Id("C")).await?;

    assert_eq!(element.text().await?, "false");
    checkbox.click().await?;
    assert_eq!(element.text().await?, "true");
    checkbox.click().await?;
    assert_eq!(element.text().await?, "false");

    Ok(())
}

#[tokio::test]
async fn test_props_default() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("D")).await?;

    assert_eq!(element.text().await?, "10");

    Ok(())
}
