// Export the scraper functionality
pub fn scrape_data(url: &str) -> anyhow::Result<()> {
    crate::bin::scraper::scrape_data(url)
}
