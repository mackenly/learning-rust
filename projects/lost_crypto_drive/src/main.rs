use chrono::{Duration, Local};
use serde_json::{Result, Value};

#[tokio::main]
async fn main() {
    println!("We've all heard it before, \"If I only knew where I put that hard drive! I had to have had hundreds of bitcoins on it!\" Well, now you can see how much you allegedly lost with this tool which shows how much an investment five years ago would be worth today.");
    // do while
    loop {
        let investment_amount: f64 = get_investment_amount();
        let five_years_ago: String = get_date_from_five_years_ago();
        let today: String = Local::now().format("%d-%m-%Y").to_string();
        let tickers: Vec<String> = vec![
            "bitcoin".to_string(),
            "ethereum".to_string(),
            "dogecoin".to_string(),
        ];

        for ticker in tickers {
            let price_five_years_ago: f64 = get_price(ticker.clone(), five_years_ago.clone())
                .await
                .unwrap();
            let price_today: f64 = get_price(ticker.clone(), today.clone()).await.unwrap();
            let investment_amount_in_crypto_five_years_ago: f64 =
                investment_amount / price_five_years_ago;
            let value_of_investment_in_crypto_today: f64 =
                investment_amount_in_crypto_five_years_ago * price_today;
            let difference: f64 = value_of_investment_in_crypto_today
                - investment_amount_in_crypto_five_years_ago * price_five_years_ago;
            println!("If you invested ${:.2} in {} five years ago, you would have {:.2} {} worth ${:.2} today. That's a difference of ${:.2} dollars.", investment_amount, ticker, investment_amount_in_crypto_five_years_ago, ticker, value_of_investment_in_crypto_today, difference);
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

fn get_date_from_five_years_ago() -> String {
    // get today's date
    let now: chrono::DateTime<Local> = Local::now();
    // subtract five years from today's date
    let five_years_ago: chrono::DateTime<Local> = now - Duration::days(1825);
    // format the date to be in the format of the API dd-mm-yyyy
    let five_years_ago: String = five_years_ago.format("%d-%m-%Y").to_string();
    five_years_ago
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
    fn test_get_date_from_five_years_ago() {
        let now: chrono::DateTime<Local> = Local::now();
        let five_years_ago: chrono::DateTime<Local> = now - Duration::days(1825);
        let five_years_ago: String = five_years_ago.format("%d-%m-%Y").to_string();
        assert_eq!(get_date_from_five_years_ago(), five_years_ago);
    }

    #[tokio::test]
    async fn test_get_price() {
        assert_eq!(
            get_price("bitcoin".to_string(), "18-11-2023".to_string())
                .await
                .unwrap(),
            36527.76022530742
        );
        assert_eq!(
            get_price("ethereum".to_string(), "18-11-2023".to_string())
                .await
                .unwrap(),
            1956.5174321166123
        );
        assert_eq!(
            get_price("dogecoin".to_string(), "18-11-2023".to_string())
                .await
                .unwrap(),
            0.08602320148882048
        );
    }
}