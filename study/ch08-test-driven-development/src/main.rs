#[derive(Clone)]
struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            results: vec![],
            current_input: String::new(),
            total: 0,
            adds: true,
        }
    }

    fn clear(&mut self) {
        self.current_input.clear();
    }

    fn push_char(&mut self, character: char) {
        self.current_input.push(character);
    }

    fn math(&mut self, input: &str) -> i32 {
        if !input
            .chars()
            .all(|character| OKAY_CHARACTERS.contains(character))
            || !input
                .chars()
                .take(2)
                .any(|character| character.is_numeric())
        {
            panic!("Please only input numbers, +-, or spaces.");
        }

        let input = input
            .trim_end_matches(|x| "+- ".contains(x))
            .chars()
            .filter(|x| *x != ' ')
            .collect::<String>(); // Remove + and - at the end, and all spaces
        for character in input.chars() {
            match character {
                '+' => {
                    if !self.current_input.is_empty() {
                        // If the string is empty, we don't want to push "" into self.results
                        self.results.push(self.current_input.clone()); // But if it's not empty, it will be a number. Push it into the vec
                        self.clear(); // Then clear the string
                    }
                }
                '-' => {
                    // If we get a -,
                    if self.current_input.contains('-') || self.current_input.is_empty() {
                        // check to see if it's empty or has a -
                        self.push_char(character) // if so, then push it in
                    } else {
                        // otherwise, it will contain a number
                        self.results.push(self.current_input.clone()); // so push the number into self.results, clear it and then push -
                        self.clear();
                        self.push_char(character);
                    }
                }
                number => {
                    // number here means "anything else that matches". We selected the name here
                    if self.current_input.contains('-') {
                        // We might have some - characters to push in first
                        self.results.push(self.current_input.clone());
                        self.clear();
                        self.push_char(number);
                    } else {
                        // But if we don't, that means we can push the number in
                        self.push_char(number);
                    }
                }
            }
        }
        self.results.push(self.current_input.clone()); // Push one last time after the loop is over. Don't need to .clone() because we don't use it anymore

        let math_iter = self.results.clone().into_iter();
        for entry in math_iter {
            // Iter through the items
            if entry.contains('-') {
                // If it has a - character, check if it's even or odd
                if entry.chars().count() % 2 == 1 {
                    self.adds = match self.adds {
                        true => false,
                        false => true,
                    };
                    continue; // Go to the next item
                } else {
                    continue;
                }
            }
            if self.adds {
                self.total += entry.parse::<i32>().unwrap(); // If there is no '-', it must be a number. So we are safe to unwrap
            } else {
                self.total -= entry.parse::<i32>().unwrap();
                self.adds = true; // After subtracting, reset adds to true.
            }
        }
        self.total // Finally, return the total
    }
}

const OKAY_CHARACTERS: &str = "1234567890+- ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 - 2"), -1);
    }
    #[test]
    fn one_minus_minus_one_is_two() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 - -1"), 2);
    }
    #[test]
    fn nine_plus_nine_minus_nine_minus_nine_is_zero() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("9+9-9-9"), 0); // This is a new test
    }
    #[test]
    fn eight_minus_nine_plus_nine_is_eight_even_with_characters_on_the_end() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("8  - 9     +9-----+++++"), 8); // This is a new test
    }
    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        let mut calc = Calculator::new();
        calc.math("7 + seven");
    }
}

fn main() {
    let mut calculator = Calculator::new();
    let result = calculator.math("1 + 1");
    println!("{result}");
}
