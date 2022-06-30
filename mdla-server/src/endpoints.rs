use actix_web::{
    get, post,
    web::{Data, Json},
    Result,
};
use chrono::{TimeZone, Utc};

use log::{info, warn};
use rand::{
    prelude::{IteratorRandom, StdRng},
    SeedableRng,
};

use mdla_lib::model::{AppError, AppState, GuessBody, GuessResponse, HintsResponse, Validation};

use crate::errors::ResponseOrError;

fn get_today_word(words: &[String]) -> String {
    // The goal here is to get a number that change everyday in order to initialise the seed of the random number generator.
    let days_since_y0 = (Utc::now() - Utc.ymd(1, 1, 1).and_hms(0, 0, 0)).num_days();
    info!("Seed init to: {days_since_y0}");

    let mut rng: StdRng = SeedableRng::seed_from_u64(days_since_y0.unsigned_abs());

    let word = words
        .iter()
        .choose(&mut rng)
        .expect("Choose a word...")
        .clone();

    info!("Today word is : {word:?}");
    word
}

#[post("/guess")]
pub async fn guess(
    data: Data<AppState>,
    guess_body: Json<GuessBody>,
) -> Result<Json<ResponseOrError<GuessResponse>>> {
    info!("Body : {guess_body:?}");

    let word: Vec<char> = get_today_word(&data.playable_word_list)
        .to_uppercase()
        .chars()
        .collect();

    let guess = guess_body.guess.to_uppercase();
    let guess_vec: Vec<char> = guess.chars().collect();

    if !data.all_word_list.contains(&guess) && guess_vec != word {
        let error = AppError::WordNotInDictionary(guess);
        warn!("{error:?}");
        return Err(ResponseOrError::<GuessResponse>::Error(error).into());
    }
    if word.len() != guess.len() {
        let error = AppError::BadWordLength {
            size_expected: word.len(),
            size_received: guess.len(),
            word_sent: guess,
        };
        warn!("{error:?}");
        return Err(ResponseOrError::<GuessResponse>::Error(error).into());
    }

    let validation_list = get_validation_list(word, guess_vec);

    let response = GuessResponse { validation_list };
    Ok(Json(ResponseOrError::Response(response)))
}

#[get("/hints")]
pub async fn hints(data: Data<AppState>) -> Result<Json<HintsResponse>> {
    let word: Vec<char> = get_today_word(&data.playable_word_list).chars().collect();

    let response = HintsResponse {
        first_letter: word[0],
        number_of_letters: word.len(),
    };
    Ok(Json(response))
}

fn get_validation_list(word: Vec<char>, guess_word: Vec<char>) -> Vec<Validation> {
    let mut validation_list = vec![];

    for (wc, gc) in Iterator::zip(word.iter(), guess_word.iter()) {
        let validation = match (wc == gc, word.contains(gc)) {
            (true, _) => Validation::Correct(*gc),
            (false, true) => Validation::Present(*gc),
            (false, false) => Validation::NotInWord(*gc),
        };

        validation_list.push(validation);
    }

    validation_list
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn test_get_validation_list_empty() {
        let result = get_validation_list(vec![], vec![]);
        assert_eq!(result, []);
    }

    #[test]
    fn test_get_validation_list_all_correct() {
        let result = get_validation_list(vec!['v', 'd', 'l', 'a', '!'], vec!['v', 'd', 'l', 'a']);
        assert_eq!(
            result,
            [
                Validation::Correct('v'),
                Validation::Correct('d'),
                Validation::Correct('l'),
                Validation::Correct('a'),
            ]
        );
    }

    #[test]
    fn test_get_validation_list_random() {
        let mut rng = thread_rng();

        let word_len: usize = rng.gen_range(1..100);

        let mut word = vec!['0'; word_len];
        let mut guess_word = vec!['0'; word_len];

        rng.fill(&mut word[..]);
        rng.fill(&mut guess_word[..]);

        let result = get_validation_list(word, guess_word.clone());

        assert!(
            Iterator::zip(result.iter(), guess_word.iter()).all(|(v, c)| {
                match v {
                    Validation::Correct(cv) => cv == c,
                    Validation::NotInWord(cv) => cv == c,
                    Validation::Present(cv) => cv == c,
                }
            })
        );
    }

    #[test]
    fn test_get_validation_list_mixed_validation_1() {
        let result =
            get_validation_list(vec!['a', 'b', 'c', 'd', 'e'], vec!['f', 'a', 'b', 'd', 'g']);
        assert_eq!(
            result,
            [
                Validation::NotInWord('f'),
                Validation::Present('a'),
                Validation::Present('b'),
                Validation::Correct('d'),
                Validation::NotInWord('g')
            ]
        );
    }

    // Todo: Make this test pass
    #[test]
    fn test_get_validation_list_mixed_validation_2() {
        let result =
            get_validation_list(vec!['v', 'd', 'l', 'a', '!'], vec!['m', 'l', 'd', 'a', 'a']);
        assert_eq!(
            result,
            [
                Validation::NotInWord('m'),
                Validation::Present('l'),
                Validation::Present('d'),
                Validation::Correct('a'),
                Validation::NotInWord('a')
            ]
        );
    }
}
