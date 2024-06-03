enum Asset {
    Stocks,
    Bonds,
    Cash,
    Funds,
}

impl Asset {
    fn price(&self) -> f64 {
        match self {
            Asset::Bonds => 20.0,
            Asset::Stocks => 10.0,
            Asset::Funds => 30.0,
            Asset::Cash => 45.0,
        }
    }
}

pub(crate) fn assets() {
    let portfolio = [Asset::Bonds, Asset::Stocks, Asset::Funds, Asset::Cash];
    let total: f64 = portfolio.iter().map(|asset| asset.price()).sum();

    println!("Total in portfolio: R$: {}", total);
}
