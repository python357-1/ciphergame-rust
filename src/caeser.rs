pub fn caesar_cipher(input: String, shift: usize) -> String{
        let alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let num_array: Vec<usize> = input.to_lowercase().chars().map(|x| match alphabet.iter().position(|&s| s == x) { Some(e) => if e+shift >= 26 { e+shift-26 } else { e+shift }, None => 2000}).collect::<Vec<usize>>();
        let cipher_text: String = num_array.iter().map(|x| if *x < 26 { alphabet[*x as usize ] } else { ' ' }).collect::<String>();
        return cipher_text
    }

    #[cfg(test)]
    mod tests {
        use crate::caeser::caesar_cipher;

        #[test]
        fn caesar_test() {
            let result1 = caesar_cipher("the quick brown fox jumped over the lazy dog".to_string(), 20);
            assert_eq!(result1, "znk waoiq hxuct lud pasvkj ubkx znk rgfe jum");

            let result2 = caesar_cipher("znk waoiq hxuct lud pasvkj ubkx znk rgfe jum".to_string(), 20);
            assert_eq!(result2, "the quick brown fox jumped over the lazy dog");
        }
    }
