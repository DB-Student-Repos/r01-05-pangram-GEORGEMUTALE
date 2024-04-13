/// Determining whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // Create a boolean array to mark if each letter is present
    let mut present = [false; 26];

    // Iterating through each character in the sentence
    for c in sentence.chars() {
        // Convert the character to lowercase
        let lower_case_c = c.to_ascii_lowercase();
        // Check if it is a lowercase English alphabet letter
        if lower_case_c.is_ascii_alphabetic() {
            // Subtract 'a' from the lowercase character to get the index in the present array
            let index = (lower_case_c as u8 - b'a') as usize;
            // Mark the letter as present
            present[index] = true;
        }
    }

    // Check if all letters are present
    present.iter().all(|&p| p)
}
