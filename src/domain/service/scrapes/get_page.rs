pub async fn get_page(url: &str) -> Result<(), Box<dyn Error>> {
    let htm = reqwest::get(url).await?;
    
}
