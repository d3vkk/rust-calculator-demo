use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calculator")]
#[command(version = "1.0")]
#[command(author = "d3vkk")]
#[command(about = "Basic Calculator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds two numbers
    Add {
        /// The first number
        a: f64,
        /// The second number
        b: f64,
    },
    /// Subtracts two numbers
    Subtract {
        /// The first number
        a: f64,
        /// The second number
        b: f64,
    },
    /// Multiplies two numbers
    Multiply {
        /// The first number
        a: f64,
        /// The second number
        b: f64,
    },
    /// Divides two numbers
    Divide {
        /// The first number
        a: f64,
        /// The second number
        b: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { a, b } => {
            println!("{}", a + b);
        }
        Commands::Subtract { a, b } => {
            println!("{}", a - b);
        }
        Commands::Multiply { a, b } => {
            println!("{}", a * b);
        }
        Commands::Divide { a, b } => {
            if b == 0.0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                println!("{}", a / b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use clap::CommandFactory;

    #[test]
    fn test_add() {
        let cli = Cli::try_parse_from(["calculator", "add", "5.0", "3.0"]).unwrap();
        match cli.command {
            Commands::Add { a, b } => {
                assert_eq!(a + b, 8.0);
            }
            _ => panic!("Expected add command"),
        }
    }

    #[test]
    fn test_subtract() {
        let cli = Cli::try_parse_from(["calculator", "subtract", "10.0", "4.0"]).unwrap();
        match cli.command {
            Commands::Subtract { a, b } => {
                assert_eq!(a - b, 6.0);
            }
            _ => panic!("Expected subtract command"),
        }
    }

    #[test]
    fn test_multiply() {
        let cli = Cli::try_parse_from(["calculator", "multiply", "7.0", "6.0"]).unwrap();
        match cli.command {
            Commands::Multiply { a, b } => {
                assert_eq!(a * b, 42.0);
            }
            _ => panic!("Expected multiply command"),
        }
    }

    #[test]
    fn test_divide() {
        let cli = Cli::try_parse_from(["calculator", "divide", "8.0", "2.0"]).unwrap();
        match cli.command {
            Commands::Divide { a, b } => {
                assert_eq!(a / b, 4.0);
            }
            _ => panic!("Expected divide command"),
        }
    }

    #[test]
    fn test_divide_by_zero() {
        let cli = Cli::try_parse_from(["calculator", "divide", "8.0", "0.0"]).unwrap();
        match cli.command {
            Commands::Divide { a: _, b } => {
                assert!(b == 0.0);
            }
            _ => panic!("Expected divide command"),
        }
    }
}
