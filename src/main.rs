use clap::Parser;
use serde::{Deserialize, Serialize};

/// CLI tool that can let you to convert currencies without leaving your terminal
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Amount of input currency
    #[clap(short, long)]
    amount: Option<f64>,
    /// Currency to convert from
    #[clap(short, short, value_parser)]
    from: String,
    /// Currency to convert to
    #[clap(short, short, value_parser)]
    to: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Rates {
    AUD: f64,
    AZN: f64,
    GBP: f64,
    AMD: f64,
    BYN: f64,
    BGN: f64,
    BRL: f64,
    HUF: f64,
    VND: f64,
    HKD: f64,
    GEL: f64,
    DKK: f64,
    AED: f64,
    USD: f64,
    EUR: f64,
    EGP: f64,
    INR: f64,
    IDR: f64,
    KZT: f64,
    CAD: f64,
    QAR: f64,
    KGS: f64,
    CNY: f64,
    MDL: f64,
    NZD: f64,
    NOK: f64,
    PLN: f64,
    RON: f64,
    XDR: f64,
    SGD: f64,
    TJS: f64,
    THB: f64,
    TRY: f64,
    TMT: f64,
    UZS: f64,
    UAH: f64,
    CZK: f64,
    SEK: f64,
    CHF: f64,
    RSD: f64,
    ZAR: f64,
    KRW: f64,
    JPY: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RatesResponse {
    rates: Rates,
}

fn parse_by_name(name: String, rates: &Rates) -> f64 {
    match name.as_str() {
        "RUB" => 1f64,
        "AUD" => rates.AUD,
        "AZN" => rates.AZN,
        "GBP" => rates.GBP,
        "AMD" => rates.AMD,
        "BYN" => rates.BYN,
        "BGN" => rates.BGN,
        "BRL" => rates.BRL,
        "HUF" => rates.HUF,
        "VND" => rates.VND,
        "HKD" => rates.HKD,
        "GEL" => rates.GEL,
        "DKK" => rates.DKK,
        "AED" => rates.AED,
        "USD" => rates.USD,
        "EUR" => rates.EUR,
        "EGP" => rates.EGP,
        "INR" => rates.INR,
        "IDR" => rates.IDR,
        "KZT" => rates.KZT,
        "CAD" => rates.CAD,
        "QAR" => rates.QAR,
        "KGS" => rates.KGS,
        "CNY" => rates.CNY,
        "MDL" => rates.MDL,
        "NZD" => rates.NZD,
        "NOK" => rates.NOK,
        "PLN" => rates.PLN,
        "RON" => rates.RON,
        "XDR" => rates.XDR,
        "SGD" => rates.SGD,
        "TJS" => rates.TJS,
        "THB" => rates.THB,
        "TRY" => rates.TRY,
        "TMT" => rates.TMT,
        "UZS" => rates.UZS,
        "UAH" => rates.UAH,
        "CZK" => rates.CZK,
        "SEK" => rates.SEK,
        "CHF" => rates.CHF,
        "RSD" => rates.RSD,
        "ZAR" => rates.ZAR,
        "KRW" => rates.KRW,
        "JPY" => rates.JPY,
        _ => 0f64,
    }
}

fn calc(amount: f64, from: String, to: String, rates: Rates) -> f64 {
    amount / parse_by_name(from, &rates) * parse_by_name(to, &rates)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Arguments::parse();

    let rates_response: RatesResponse = reqwest::get("https://www.cbr-xml-daily.ru/latest.js")
        .await?
        .json()
        .await?;

    let rates = rates_response.rates;

    println!(
        "{}",
        calc(args.amount.unwrap_or(1f64), args.from, args.to, rates)
    );

    Ok(())
}
