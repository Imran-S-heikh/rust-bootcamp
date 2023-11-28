use error_stack::{Context, Report, Result, ResultExt};
use std::{collections::HashMap, error::Error, fmt, io};

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

#[derive(Debug)]
struct ParsePaymentInfoError;

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Parsing payment error: invalid payment info")
    }
}

// impl Error for ParsePaymentInfoError {}

impl Context for ParsePaymentInfoError {}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other,
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("\nCredit card error: Could not retrive credit card")
    }
}

impl Context for CreditCardError {}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse::<u32>()
                .attach_printable_lazy(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!("Failed to parse input as numbers. Input: {card}"))?;

    Ok(numbers)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError).attach_printable(format!(
            "Incorrect number of elements parsed. Expected: {expected_len}, got: {len}"
        )));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card found for {name}");
        Report::new(CreditCardError::InvalidInput(msg.clone())).attach_printable(msg.clone())
    })?;
    let card = parse_card(&card_string)
        .change_context(CreditCardError::Other)
        .attach_printable(format!("{name}'s card could not be parsed."))?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("Credit Card Info: {card:?}"),
        Err(e) => {
            match e.current_context() {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other => println!("Something went wrong! Try again."),
            }

            log::error!("{e:?}")
        }
    }
}
