use serde::Deserialize;
use serde::Serialize;



#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TokenSocial {
   
    #[serde(rename = "totalSupply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "marketCap")]
    pub market_cap: Option<f64>,
    #[serde(rename = "socialWebsite")]
    pub social_website: Option<String>,
    #[serde(rename = "socialDiscord")]
    pub social_discord: Option<String>,
    #[serde(rename = "socialTelegram")]
    pub social_telegram: Option<String>,
    #[serde(rename = "socialTwitter")]
    pub social_twitter: Option<String>,
      
}
