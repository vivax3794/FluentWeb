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
async fn test_click_event() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let button = driver.find(By::Id("TARGET")).await?;
    assert_eq!(button.text().await?, "0");


    for i in 1..=5 {
        button.click().await?;
        assert_eq!(button.text().await?, i.to_string());
    } 
    
    Ok(())
}

#[tokio::test]
async fn test_custom_event() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let button = driver.find(By::Id("SUB_TARGET")).await?;
    let result = driver.find(By::Id("OTHER_TARGET")).await?;

    assert_eq!(result.text().await?, "0");
    button.click().await?;
    assert_eq!(result.text().await?, "10");
    
    Ok(())
}