//  a command-line tool that instantly generates fresh story ideas for writers, game masters, and daydreamers.
use reqwest::Client;
use clap::{Parser};
const API_URL: &str = "https://ai.hackclub.com/chat/completions";
const PROMPT: &str = "You are a creative story seed generator for a CLI tool called 'storyline'.\n\
Always return exactly one story idea in the following format, no extra text:\n\
\n\
Title: <Short, catchy title>\n\
Genre: <One or two genres>\n\
Premise: <1–2 sentences setting up the scenario>\n\
Twist: <One sentence describing an unexpected turn>\n\
\n\
Constraints:\n\
- Genre must be: {genre}\n\
- Number of twists: {twist}\n\
- Never use more than 60 words in total.\n\
- Title must be 3–6 words.\n\
- Premise should sound intriguing and vivid.\n\
- Twist(s) must be surprising but plausible.\n\
- Avoid clichés like 'it was all a dream' or 'the villain was the hero's father'.\n\
- No explanations or meta comments; output only in the above format.";

/// A command-line tool that instantly generates fresh story ideas for writers, game masters, and daydreamers.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Genre of the story idea
    #[arg(short, long, default_value = "any")]
    genre: String,
    /// The number of twists in the story idea
    #[arg(short, long, default_value_t = 1)]
    twist: usize,
    /// Include reasoning in the output
    #[arg(long, default_value_t = false)]
    include_reasoning: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let genre = args.genre;
    let twist = args.twist;
    let include_reasoning = args.include_reasoning;
    let prompt = PROMPT.replace("{genre}", &genre)
                       .replace("{twist}", &twist.to_string());
    let model = "openai/gpt-oss-120B";
    let story_idea = query_ai(&prompt, model, include_reasoning).await;
    println!("{}", story_idea);
}

async fn query_ai(prompt: &str, model: &str, include_reasoning: bool) -> String {
    // post request to AI service
    let client = Client::new();
    let response = client.post(API_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_completion_tokens": 1000,
            "temperature": 0.7,
            "include_reasoning": include_reasoning,
        }))
        .send()
        .await.expect("Failed to send request");
    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await.expect("Failed to parse response");
        if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
            content.to_string()
        } else {
            "No content returned".to_string()
        }
    } else {
        format!("Error: {}", response.status())
    }
}