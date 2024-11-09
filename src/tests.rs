use super::*;
use serde::{Deserialize, Serialize};

use tokio; // For async tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use super::apis::Error;
    use super::apis::configuration;
    use super::apis::openbook_v2_api::api_get_markets_v2; // Adjust based on your module structure
    use crate::config::API_CLIENT;
    use crate::models::ApiGetMarketsResponseV2;
    #[tokio::test]
    async fn test_api_get_markets_v2() {


        match API_CLIENT.get("api/v2/openbook/markets").await {
            Ok(data) => {
                println!("Market data: {}", data);
                let response : ApiGetMarketsResponseV2 = serde_json::from_str(&data).expect("JSON was not well-formatted");
                assert!(
                    response.markets.as_ref().map_or(false, |markets| !markets.is_empty()),
                    "Markets list should not be empty"
                );
                if let Some(markets) = response.markets.as_ref() {
                    for (key, market) in markets {
                        // Use `unwrap_or` to provide a default if address is None
                        let address = market.address.as_deref().unwrap_or("No address provided");
                        println!("Market Key: {}, Market Address: {}", key, address);
                        // Print other fields of `ApiMarketV2` as needed
                    }
                }                // assert_eq!(response.markets[0].id, "market1");
                // assert_eq!(response.markets[0].name, "Market 1");

            },
            Err(err) => eprintln!("Error fetching market data: {}", err),
        }


//         // Step 1: Configure the API settings
//         let config = configuration::Configuration::default();
//  //       config.base_path = "https://api.example.com".to_string(); // Replace with the actual base URL

//         // Step 2: Call the `api_get_markets_v2` function
//         match api_get_markets_v2(&config).await {
//             Ok(response) => {
//                 // Step 3: Verify the response
//                 assert!(
//                     response.markets.as_ref().map_or(false, |markets| !markets.is_empty()),
//                     "Markets list should not be empty"
//                 );
//                 if let Some(markets) = response.markets.as_ref() {
//                     for (key, market) in markets {
//                         // Use `unwrap_or` to provide a default if address is None
//                         let address = market.address.as_deref().unwrap_or("No address provided");
//                         println!("Market Key: {}, Market Address: {}", key, address);
//                         // Print other fields of `ApiMarketV2` as needed
//                     }
//                 }                // assert_eq!(response.markets[0].id, "market1");
//                 // assert_eq!(response.markets[0].name, "Market 1");
//             },
//             Err(e) => {
//                 panic!("API call failed with error: {:?}", e);
//             }
//         }
    }
}
