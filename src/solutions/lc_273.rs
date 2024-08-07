use crate::structs::solution::Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        if num < 0 {
            return format!("Minus {}", Self::number_to_words(-num));
        }

        let units = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        let teens = [
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        let tens = [
            "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let thousands = ["", "Thousand", "Million", "Billion"];

        let mut n = num;
        let mut words = String::new();
        let mut chunk_count = 0;

        while n > 0 {
            let chunk = n % 1000;
            if chunk != 0 {
                let chunk_words = three_digits_to_words(chunk, &units, &teens, &tens);
                if chunk_count > 0 {
                    words = format!("{} {} {}", chunk_words, thousands[chunk_count], words);
                } else {
                    words = format!("{} {}", chunk_words, words);
                }
            }
            n /= 1000;
            chunk_count += 1;
        }

        words.trim().to_string()
    }
}

fn three_digits_to_words(n: i32, units: &[&str], teens: &[&str], tens: &[&str]) -> String {
    assert!(n < 1000);

    let hundred = n / 100;
    let rem = n % 100;
    let mut word = String::new();

    if hundred > 0 {
        word = format!("{} Hundred", units[hundred as usize]);
    }

    if rem > 0 {
        if !word.is_empty() {
            word.push(' ')
        };
        if rem < 10 {
            word.push_str(units[rem as usize]);
        } else if rem > 10 && rem < 20 {
            word.push_str(teens[(rem - 11) as usize]);
        } else {
            word.push_str(tens[(rem / 10) as usize]);
            if rem % 10 > 0 {
                word.push(' ');
                word.push_str(units[(rem % 10) as usize]);
            }
        }
    } else if rem == 10 {
        if !word.is_empty() {
            word.push(' ')
        };
        word.push_str(tens[1]);
    }

    word
}
