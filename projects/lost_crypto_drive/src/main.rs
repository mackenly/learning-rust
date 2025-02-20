use chrono::{Duration, Local};
use serde_json::{Result, Value};

// Notice: The API I was using seems to have became more restrictive
// Ex: It doesn't allow historical data for over a year ago
// And it seems highliy rate limited
// So this might not work and needs to be updated to work with a different API

#[tokio::main]
async fn main() {
    println!("We've all heard it before, \"If I only knew where I put that hard drive! I had to have had hundreds of bitcoins on it!\" Well, now you can see how much you allegedly lost with this tool which shows how much an investment a year ago would be worth today.");
    // do while
    loop {
        let investment_amount: f64 = get_investment_amount();
        let a_year_ago: String = get_date_from_a_year_ago();
        let today: String = Local::now().format("%d-%m-%Y").to_string();
        let tickers: Vec<String> = vec![
            "bitcoin".to_string(),
            "ethereum".to_string(),
            "dogecoin".to_string(),
        ];

        for ticker in tickers {
            let price_a_year_ago: f64 = get_price(ticker.clone(), a_year_ago.clone())
                .await
                .unwrap();
            let price_today: f64 = get_price(ticker.clone(), today.clone()).await.unwrap();
            let investment_amount_in_crypto_a_year_ago: f64 =
                investment_amount / price_a_year_ago;
            let value_of_investment_in_crypto_today: f64 =
                investment_amount_in_crypto_a_year_ago * price_today;
            let difference: f64 = value_of_investment_in_crypto_today
                - investment_amount_in_crypto_a_year_ago * price_a_year_ago;
            println!("If you invested ${:.2} in {} five years ago, you would have {:.2} {} worth ${:.2} today. That's a difference of ${:.2} dollars.", investment_amount, ticker, investment_amount_in_crypto_a_year_ago, ticker, value_of_investment_in_crypto_today, difference);
        }

        let input: String = prompt_for_input("\nWould you like to go again? (y/n)");
        if input == "n" {
            break;
        } else {
            clear_screen();
            println!("Alright, as a reminder, you can always press Ctrl+C to exit the program and the definition of insanity is doing the same thing over and over again and expecting different results.")
        }
    }
}

fn get_investment_amount() -> f64 {
    let mut investment_amount: String = String::new();
    println!("How much did you invest (USD)?");
    std::io::stdin()
        .read_line(&mut investment_amount)
        .expect("Failed to read line");
    let investment_amount: f64 = investment_amount
        .trim()
        .parse()
        .expect("Please type a number!");
    investment_amount
}

fn get_date_from_a_year_ago() -> String {
    // get today's date
    let now: chrono::DateTime<Local> = Local::now();
    // subtract five years from today's date
    let a_year_ago: chrono::DateTime<Local> = now - Duration::days(364);
    // format the date to be in the format of the API dd-mm-yyyy
    let a_year_ago: String = a_year_ago.format("%d-%m-%Y").to_string();
    a_year_ago
}

async fn http_request(url: &str) -> String {
    let resp: reqwest::Response = reqwest::get(url).await.expect("Failed to send request");
    let body: String = resp.text().await.expect("Failed to get response text");
    body
}

fn parse_json(json: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(json)?;
    Ok(v)
}

async fn get_price(ticker: String, date: String) -> Result<f64> {
    let url: String = if date == "today" {
        format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            ticker
        )
    } else {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/history?date={}",
            ticker, date
        )
    };
    let body: String = http_request(&url).await;
    let json: Value = parse_json(&body).unwrap();
    let price: f64 = if date == "today" {
        json[ticker]["usd"].as_f64().unwrap()
    } else {
        json["market_data"]["current_price"]["usd"]
            .as_f64()
            .unwrap()
    };
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(price)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn prompt_for_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    clear_screen();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_date_from_a_year_ago() {
        let now: chrono::DateTime<Local> = Local::now();
        let a_year_ago: chrono::DateTime<Local> = now - Duration::days(364);
        let a_year_ago: String = a_year_ago.format("%d-%m-%Y").to_string();
        assert_eq!(get_date_from_a_year_ago(), a_year_ago);
    }

    #[tokio::test]
    async fn test_get_price() {
        assert_eq!(
            get_price("bitcoin".to_string(), "18-11-2024".to_string())
                .await
                .unwrap(),
            89841.47194135105
        );
        assert_eq!(
            get_price("ethereum".to_string(), "18-11-2024".to_string())
                .await
                .unwrap(),
            3075.534532601624
        );
        assert_eq!(
            get_price("dogecoin".to_string(), "18-11-2024".to_string())
                .await
                .unwrap(),
            0.36707674107673377
        );
    }
}