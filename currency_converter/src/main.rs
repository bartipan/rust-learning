use std::io;

fn main() {
    let currency_type_from: CurrencyType = input_currency_type_from();
    let amount: f64 = input_amount();
    let currency_type_to: CurrencyType = input_currency_type_to();

    let currency = Currency {
        amount,
        currency_type: currency_type_from,
    };

    let result: Currency = currency.convert(&currency_type_to);

    println!(
        "The amount of exchanged currency is: {:.3} {:?}",
        result.amount, result.currency_type
    );
}

fn input_amount() -> f64 {
    println!("Insert amount you would like to exchange:");

    loop {
        let mut amount = String::new();

        io::stdin()
            .read_line(&mut amount)
            .expect("Error reading the input");

        match amount.trim().parse() {
            Ok(i) => break i,
            Err(_) => println!("Please, enter the amount of money to change."),
        }
    }
}

fn input_currency_type_from() -> CurrencyType {
    println!("Insert the currency you would like to change from (USD, EUR, CZK):");

    input_currency_type()
}

fn input_currency_type_to() -> CurrencyType {
    println!("Insert the currency you would like to change into (USD, EUR, CZK):");

    input_currency_type()
}

fn input_currency_type() -> CurrencyType {
    loop {
        let mut currency_type = String::new();

        io::stdin()
            .read_line(&mut currency_type)
            .expect("Error reading the input");

        match currency_type.trim().to_uppercase().as_str() {
            "USD" => break CurrencyType::USD,
            "EUR" => break CurrencyType::EUR,
            "CZK" => break CurrencyType::CZK,
            _ => println!("Please, enter a valid currency type (USD, EUR, CZK)!"),
        }
    }
}

struct Currency {
    amount: f64,
    currency_type: CurrencyType,
}

#[derive(Debug)]
enum CurrencyType {
    CZK,
    EUR,
    USD,
}

impl Currency {
    const USD_RATE: f64 = 1.0;
    const EUR_RATE: f64 = 0.92;
    const CZK_RATE: f64 = 23.45;

    fn convert(&self, exchange_to: &CurrencyType) -> Currency {
        match exchange_to {
            CurrencyType::CZK => self.convert_to_czk(),
            CurrencyType::EUR => self.convert_to_eur(),
            CurrencyType::USD => self.convert_to_usd(),
        }
    }

    fn convert_to_czk(&self) -> Currency {
        let mut amount = self.amount;

        amount = match self.currency_type {
            CurrencyType::CZK => amount,
            CurrencyType::EUR => amount * (Currency::CZK_RATE / Currency::EUR_RATE),
            CurrencyType::USD => amount * (Currency::CZK_RATE / Currency::USD_RATE),
        };

        Currency {
            amount,
            currency_type: CurrencyType::CZK,
        }
    }

    fn convert_to_eur(&self) -> Currency {
        let mut amount = self.amount;

        amount = match self.currency_type {
            CurrencyType::CZK => amount * (Currency::EUR_RATE / Currency::CZK_RATE),
            CurrencyType::EUR => amount,
            CurrencyType::USD => amount * (Currency::EUR_RATE / Currency::USD_RATE),
        };

        Currency {
            amount,
            currency_type: CurrencyType::EUR,
        }
    }

    fn convert_to_usd(&self) -> Currency {
        let mut amount = self.amount;

        amount = match self.currency_type {
            CurrencyType::CZK => amount * (Currency::USD_RATE / Currency::CZK_RATE),
            CurrencyType::EUR => amount * (Currency::USD_RATE / Currency::EUR_RATE),
            CurrencyType::USD => amount,
        };

        Currency {
            amount,
            currency_type: CurrencyType::USD,
        }
    }
}
