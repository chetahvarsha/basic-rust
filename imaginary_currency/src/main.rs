mod data;

use data::Currency;

fn main() {
    let mut ic = 0.0;

    let currencies = vec![
        Currency::USD { gdp: 24600000.0, ex_rate: 1.0 },
        Currency::CNY { gdp: 14300000.0, ex_rate: 7.2543 },
        Currency::SAR { gdp: 1030000.0, ex_rate: 3.7501 },
        Currency::AED { gdp: 414000.0, ex_rate: 3.6725 },
        Currency::INR { gdp: 2760000.0, ex_rate: 83.4200 },
        Currency::RUB { gdp: 1780000.0, ex_rate: 74.3825 },
        Currency::BRL { gdp: 2060000.0, ex_rate: 5.4054 },
        Currency::EUR { gdp: 14100000.0, ex_rate: 1.0739},
        Currency::ZAR { gdp: 340000.0, ex_rate: 18.0691 },
        Currency::SGD { gdp: 527000.0, ex_rate: 1.3515 },
        Currency::THB { gdp: 534000.0, ex_rate: 36.6700},
        Currency::MYR { gdp: 315000.0, ex_rate: 4.7119 },
        Currency::IDR { gdp: 1130000.0, ex_rate: 14.831 },
        Currency::AUD { gdp: 1230000.0, ex_rate: 0.6652 },
    ];

    for currency in &currencies {
        let ex_rate = match currency {
            Currency::USD { ex_rate, .. } => ex_rate,
            Currency::CNY { ex_rate, .. } => ex_rate,
            Currency::SAR { ex_rate, .. } => ex_rate,
            Currency::AED { ex_rate, .. } => ex_rate,
            Currency::INR { ex_rate, .. } => ex_rate,
            Currency::RUB { ex_rate, .. } => ex_rate,
            Currency::BRL { ex_rate, .. } => ex_rate,
            Currency::EUR { ex_rate, .. } => ex_rate,
            Currency::ZAR { ex_rate, .. } => ex_rate,
            Currency::SGD { ex_rate, .. } => ex_rate,
            Currency::THB { ex_rate, .. } => ex_rate,
            Currency::MYR { ex_rate, .. } => ex_rate,
            Currency::IDR { ex_rate, .. } => ex_rate,
            Currency::AUD { ex_rate, .. } => ex_rate,
        };

        ic += currency.weight(&currencies) * ex_rate;
    }

    
    println!("the new currency is = {}", ic);
}