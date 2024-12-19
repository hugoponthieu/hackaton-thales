use std::collections::HashMap;
use reqwest::Client;
use tokio;

async fn generate_20_char_strings() {
    let charset: Vec<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .collect();

    let mut current: Vec<usize> = vec![0; 20]; // Stores indexes into charset for the 20 characters
    let mut result_string = String::new(); // String to store found characters

    loop {
        // Construct the current string from the current indices
        let current_string: String = current
            .iter()
            .map(|&index| charset[index])
            .collect();

        // Check condition via API call
        let mut map = HashMap::new();
        map.insert("content", current_string.clone());

        let mut resBody: String = "".to_string();
        let thread_join_handle = tokio::spawn(async move {
            let client: Client = Client::new();
            let res = client
                .post("https://rust-checker.hackathon.dopolytech.fr/check")
                .json(&map)
                .send()
                .await;
            resBody = res.unwrap().text().await.unwrap().clone();
        });

        thread_join_handle.await.unwrap();

        if resBody.contains("\"result\":0") {
            result_string.push_str(&current_string); // Append current string to result
            println!("Current result string: {}", result_string); // Print updated result string
        }

        // Increment the indices
        let mut position = 19; // Start incrementing from the last character

        while position < current.len() {
            if current[position] + 1 < charset.len() {
                current[position] += 1; // Increment the current position
                break;
            } else {
                current[position] = 0; // Reset this position and carry to the next
                if position == 0 {
                    return; // If we've carried past the first character, we're done
                }
                position -= 1;
            }
        }
    }
}

fn main() {
    generate_20_char_strings();
}
