mod print;
mod skatteverket;
use inquire::{validator::Validation, Text};
use print::print_logo;
use skatteverket::fetch_data_from_api;

use crate::print::print_table;

// https://skatteverket.entryscape.net/rowstore/dataset/b4de7df7-63c0-4e7e-bb59-1f156a591763?testpersonnummer=1990.*&_limit=100&_offset=0
#[tokio::main]
async fn main() {
    print_logo();
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid(
                "You're only allowed 140 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let year = Text::new("Provide a date which year/month/day the person should be born \n")
        .with_validator(validator)
        .prompt();

    let amount = Text::new("How many SSNs do you need? \n").prompt();
    match year {
        Ok(year) => {
            println!("Your year is being published... {:?}", year);
            match amount {
                Ok(amount) => {
                    println!("Amount... {:?}", amount);
                    // Call the service function
                    let result = fetch_data_from_api(year, amount).await;

                    match result {
                        Ok(api_response) => {
                            print_table(api_response.results);
                        }
                        Err(e) => print!("{}", e),
                    }
                }
                Err(err) => println!("Error while getting amount: {}", err),
            }
        }
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
