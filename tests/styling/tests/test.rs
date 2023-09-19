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
async fn test_simple_style() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("A")).await?;
    assert_eq!(element.css_value("color").await?, "rgba(255, 0, 0, 1)");
    
    Ok(())
}

#[tokio::test]
async fn test_static_var() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("B")).await?;
    assert_eq!(element.css_value("color").await?, "rgba(0, 255, 0, 1)");
    
    Ok(())
}

#[tokio::test]
async fn test_dynamic_var() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("C")).await?;
    
    for i in 0..=5 {
        assert_eq!(element.css_value("color").await?, format!("rgba({i}, 0, 0, 1)"));
        element.click().await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_scoped_css() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element_main = driver.find(By::Id("D1")).await?;
    assert_eq!(element_main.css_value("color").await?, "rgba(0, 0, 255, 1)");
    
    let element_sub = driver.find(By::Id("D2")).await?;
    assert_ne!(element_sub.css_value("color").await?, "rgba(0, 0, 255, 1)");

    Ok(())
}

#[tokio::test]
async fn test_static_var_from_parent() -> WebDriverResult<()> {
    let driver = create_driver().await;

    let element = driver.find(By::Id("E")).await?;
    assert_eq!(element.css_value("color").await?, "rgba(255, 0, 0, 1)");
    
    Ok(())
}