pub fn main() {

}

struct Solution;

impl Solution {
    pub fn longest_palindromic_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();

        if s.len() == 2 && s[0] != s[1] { return s[0].to_string() };

        let mut palindrome = String::new();

        for i in 0..s.len() {
            let mut left = i;
            let mut right = i;

            while right < s.len() {
                if s[left] != s[right] { break; }
                let new_palin = s[left..=right].iter().collect::<String>();
                if new_palin.len() > palindrome.len() { palindrome = new_palin };
                if let Some(new_left) = left.checked_sub(1) {
                    left = new_left;
                    right += 1;
                } else {
                    break;
                }
            }

            left = i;
            right = i;
            
            while right < s.len() - 1 {
                println!("{} {}", s[left], s[right + 1]);
                if s[left] != s[right + 1] { break; }
                let new_palin = s[left..=right + 1].iter().collect::<String>();
                if new_palin.len() > palindrome.len() { palindrome = new_palin };
                if let Some(new_left) = left.checked_sub(1) {
                    left = new_left;
                    right += 1;
                } else {
                    break;
                }
            }


            
        }

        palindrome
    }
}

#[cfg(test)]
pub mod test {
    use crate::Solution;

    #[test]
    fn first_case() {
        let result = Solution::longest_palindromic_substring("babad".to_string());
        assert_eq!(result, "bab");
    }

    #[test]
    fn second_case() {
        let result = Solution::longest_palindromic_substring("cbbd".to_string());
        assert_eq!(result, "bb");
    }

    #[test]
    fn third_case() {
        let result = Solution::longest_palindromic_substring("a".to_string());
        assert_eq!(result, "a");
    }

    #[test]
    fn fourth_case() {
        let result = Solution::longest_palindromic_substring("ac".to_string());
        assert_eq!(result, "a");
    }

    #[test]
    fn fifth_case() {
        let result = Solution::longest_palindromic_substring("bb".to_string());
        assert_eq!(result, "bb");
    }

    #[test]
    fn sixth_case() {
        let result = Solution::longest_palindromic_substring("abb".to_string());
        assert_eq!(result, "bb");
    }
}
