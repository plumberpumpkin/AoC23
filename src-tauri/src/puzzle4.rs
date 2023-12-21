#[tauri::command]
pub fn puzzle4(input: String) -> u32 {
    let mut points: u32 = 0;
    let intermediate_input: Vec<&str> = input.split("\n").collect();
    let cards: Vec<Card> = get_cards(intermediate_input);
    for card in cards{
        let winning_numbers = card.winning_numbers;
        let card_numbers = card.card_numbers;
        let mut number_of_winning_numbers: u32  = 0;
        for card_number in card_numbers{
            if winning_numbers.contains(&card_number){
                number_of_winning_numbers += 1;
            }
        }
        println!("Card {} has {} winning numbers", card.card_number, number_of_winning_numbers);
        if number_of_winning_numbers == 0 {
            points += 0;
        } else{
            let mut points_for_card: u32 = 0;
            for i in 0..number_of_winning_numbers{
                if points_for_card == 0 {
                    points_for_card += 1;
                } else{
                    points_for_card *= 2;
                }
            }
            points += points_for_card;
        }

    }

    return points;
}

struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn get_cards(input: Vec<&str>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for raw_card in input {
        let mut card: Card = Card {
            card_number: 0,
            winning_numbers: Vec::new(),
            card_numbers: Vec::new(),
        };
        let raw_card_intermediate_info: Vec<&str> = raw_card.split(":").collect();
        card.card_number = raw_card_intermediate_info[0]
            .replace(" ", "")
            .replace("Card", "")
            .parse::<u32>()
            .unwrap();
        let raw_card_numbers_complete: Vec<&str> = raw_card_intermediate_info[1]
            .split("|")
            .collect();
        let mut card_numbers: Vec<u32> = Vec::new();
        let mut card_winning_numbers: Vec<u32> = Vec::new();
        let raw_card_numbers:Vec<&str> = raw_card_numbers_complete[1]
            .split_whitespace()
            .collect();
        let raw_card_winning_numbers:Vec<&str> = raw_card_numbers_complete[0]
            .split_whitespace()
            .collect();
        for raw_card_number in raw_card_numbers {
            card_numbers.push(raw_card_number.parse::<u32>().unwrap());
        }
        for raw_card_winning_number in raw_card_winning_numbers {
            card_winning_numbers.push(raw_card_winning_number.parse::<u32>().unwrap());
        }
        card.card_numbers = card_numbers;
        card.winning_numbers = card_winning_numbers;
        cards.push(card);
    }
    return cards;
}
