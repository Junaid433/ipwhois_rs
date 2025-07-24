use crate::models::IpWhoIsResponse;
use crate::errors::IpWhoisError;
use reqwest::Client;

pub struct IpWhoIs {
    pub info: Option<IpWhoIsResponse>,
}

impl IpWhoIs {
    pub fn new() -> Self {
        Self { info: None }
    }

    pub async fn lookup(&mut self, ip: &str) -> Result<IpWhoIsResponse, IpWhoisError> {
        let client = Client::new();
        let url = format!("https://ipwho.is/{}", ip);
        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(IpWhoisError::ApiError(format!("Unexpected HTTP status: {}", response.status())));
        }

        let data = response.json::<IpWhoIsResponse>().await?;

        if !data.success {
            return Err(IpWhoisError::ApiError(format!("API returned failure for IP {}", ip)));
        }

        self.info = Some(data.clone());
        Ok(data)
    }
}
