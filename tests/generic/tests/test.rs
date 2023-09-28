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
async fn test_simple_with_default() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let u8_element = driver.find(By::Id("A1")).await?;
    assert_eq!(u8_element.text().await?, "0");

    let option_element = driver.find(By::Id("A2")).await?;
    assert_eq!(option_element.text().await?, "false");

    Ok(())
}

#[tokio::test]
async fn test_default() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("B1")).await?;
    let button = element.find(By::Tag("button")).await?;
    let p = element.find(By::Tag("p")).await?;

    button.click().await?;
    assert_eq!(p.text().await?, "0");

    let element = driver.find(By::Id("B2")).await?;
    let button = element.find(By::Tag("button")).await?;
    let p = element.find(By::Tag("p")).await?;

    button.click().await?;
    assert_eq!(p.text().await?, "false");

    Ok(())
}

#[tokio::test]
async fn test_props() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let u8_element = driver.find(By::Id("C1")).await?;
    assert_eq!(u8_element.text().await?, "100");

    let option_element = driver.find(By::Id("C2")).await?;
    assert_eq!(option_element.text().await?, "true");

    Ok(())
}
