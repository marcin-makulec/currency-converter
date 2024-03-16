use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Source currency code
    from: String,

    /// Target currency code
    to: String,

    /// Amount of currency to convert
    #[arg(value_parser = validate_amount)]
    amount: f64,
}

fn validate_amount(s: &str) -> Result<f64, String> {
    let amount = s
        .parse::<f64>()
        .map_err(|_| format!("invalid float literal"))?;
    if amount > 0.0 {
        Ok(amount)
    } else {
        Err("must be positive".into())
    }
}

fn main() {
    let cli = Cli::parse();

    println!("from: {}", cli.from);
    println!("to: {}", cli.to);
    println!("amount: {}", cli.amount);
}
