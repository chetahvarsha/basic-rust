pub enum Currency {
    USD { gdp: f64, ex_rate: f64 },
    CNY { gdp: f64, ex_rate: f64 },
    SAR { gdp: f64, ex_rate: f64 },
    AED { gdp: f64, ex_rate: f64 },
    INR { gdp: f64, ex_rate: f64 },
    RUB { gdp: f64, ex_rate: f64 },
    BRL { gdp: f64, ex_rate: f64 },
    EUR { gdp: f64, ex_rate: f64 },
    ZAR { gdp: f64, ex_rate: f64 },
    SGD { gdp: f64, ex_rate: f64 },
    THB { gdp: f64, ex_rate: f64 },
    MYR { gdp: f64, ex_rate: f64 },
    IDR { gdp: f64, ex_rate: f64 },
    AUD { gdp: f64, ex_rate: f64 },
}

impl Currency {
    pub fn weight(&self, currencies: &Vec<Currency>) -> f64 {
        let total_gdp = currencies.iter().fold(0.0, |acc, currency| {
            match currency {
                Currency::USD { gdp, .. } => acc + gdp,
                Currency::CNY { gdp, .. } => acc + gdp,
                Currency::SAR { gdp, .. } => acc + gdp,
                Currency::AED { gdp, .. } => acc + gdp,
                Currency::INR { gdp, .. } => acc + gdp,
                Currency::RUB { gdp, .. } => acc + gdp,
                Currency::BRL { gdp, .. } => acc + gdp,
                Currency::EUR { gdp, .. } => acc + gdp,
                Currency::ZAR { gdp, .. } => acc + gdp,
                Currency::SGD { gdp, .. } => acc + gdp,
                Currency::THB { gdp, .. } => acc + gdp,
                Currency::MYR { gdp, .. } => acc + gdp,
                Currency::IDR { gdp, .. } => acc + gdp,
                Currency::AUD { gdp, .. } => acc + gdp,
            }
        });
        println!("the new currency is = {}",total_gdp);
        match self {
            Currency::USD { gdp, .. } => gdp / total_gdp,
            Currency::CNY { gdp, .. } => gdp / total_gdp,
            Currency::SAR { gdp, .. } => gdp / total_gdp,
            Currency::AED { gdp, .. } => gdp / total_gdp,
            Currency::INR { gdp, .. } => gdp / total_gdp,
            Currency::RUB { gdp, .. } => gdp / total_gdp,
            Currency::BRL { gdp, .. } => gdp / total_gdp,
            Currency::EUR { gdp, .. } => gdp / total_gdp,
            Currency::ZAR { gdp, .. } => gdp / total_gdp,
            Currency::SGD { gdp, .. } => gdp / total_gdp,
            Currency::THB { gdp, .. } => gdp / total_gdp,
            Currency::MYR { gdp, .. } => gdp / total_gdp,
            Currency::IDR { gdp, .. } => gdp / total_gdp,
            Currency::AUD { gdp, .. } => gdp / total_gdp,
        }
    }
    
}