use std::io;
use rand::seq::SliceRandom;

struct Question {
    hint: &'static str,
    answer: &'static str,
}

fn main() {
    let questions = vec![
        Question { hint: "It shines in the sky during the day", answer: "sun" },
        Question { hint: "You use this to open a locked door", answer: "key" },
        Question { hint: "It barks and is a common pet", answer: "dog" },
        Question { hint: "The red fruit that keeps doctors away", answer: "apple" },
        Question { hint: "It flies and delivers letters in cartoons", answer: "pigeon" },
        Question { hint: "The planet we live on", answer: "earth" },
        Question { hint: "It tells you the time", answer: "clock" },
        Question { hint: "You write with it and it has ink", answer: "pen" },
        Question { hint: "It comes after Tuesday", answer: "wednesday" },
        Question { hint: "The opposite of cold", answer: "hot" },
        Question { hint: "You wear these on your feet", answer: "shoes" },
        Question { hint: "Insects that make honey", answer: "bees" },
        Question { hint: "A flying vehicle with wings", answer: "airplane" },
        Question { hint: "It grows on your head", answer: "hair" },
        Question { hint: "You sleep on it", answer: "bed" },
        Question { hint: "The color of grass", answer: "green" },
        Question { hint: "You drink it to stay hydrated", answer: "water" },
        Question { hint: "Used to cut paper", answer: "scissors" },
        Question { hint: "Frozen water", answer: "ice" },
        Question { hint: "Big body of saltwater", answer: "ocean" },
    ];


    let mut score = 0;
    let total = 3;
    let mut rng = rand::thread_rng();
    let selected: Vec<_> = questions.choose_multiple(&mut rng, total).collect();

    println!("Welcome to the Hint → Answer guessing game!");
    println!("You will get {} questions. Type your answer and press Enter.\n", total);

    for (i, q) in selected.iter().enumerate() {
        println!("Hint {}: {}", i + 1, q.hint);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let guess = input.trim().to_lowercase();

        if guess == q.answer {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Wrong! The answer was: {}\n", q.answer);
        }
    }

    println!("🎯 Game Over! You scored {}/{}.", score, total);
}
