use std::env;
use std::thread::JoinHandle;

fn main() {
    println!("Enter word to count number of chose word in a phrase");

    // The args takes commands and checks that something is provided. If not, it returns an error message.
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("No word provided to count.");
        return;
    }

    // Join all arguments except the last one as the input string (last argument is number of threads)
    let word_from_command = args[1..args.len()-2].join(" ");

    
    // Parse the second argument as number of threads. If no number is provided, the default is 4 threads.
    let num_threads = args[args.len()-2].parse::<usize>().unwrap_or(4);

    // The chosen word to count is the last argument
    let chosen_word = args[args.len()-1].to_string();


    // divide words into chunks for each thread
    let words: Vec<&str> = word_from_command.split_whitespace().collect();
    
    // If words is empty, return early
    if words.is_empty() {
        eprintln!("Input word is too short to split into chunks.");
        return;
    }
    // Chunk size is determined by dividing total words by number of threads
    let chunk_size = (words.len() + num_threads - 1) / num_threads;


    // Create a vector to store thread handles
    let mut thread_storing: Vec<JoinHandle<usize>> = Vec::new();

    // For each chunk, chunk_string will join the words in that chunk and spawn a thread to count occurrences of "the"
    for chunk in words.chunks(chunk_size) {
        let chunk_string = chunk.join(" ");
        let chosen_word = chosen_word.clone(); // cloning so that each thread has its own copy (ownership rule which says data can only be owned by one thread at a time)

        // This spawns a new thread for each chunk
        let handle = std::thread::spawn(move || {
            // Count occurrences of the chosen word in the chunk and print the count for that chunk
            let count = chunk_string.matches(&chosen_word).count();
            println!("Count in chunk: {}", count);
            count
        });
        thread_storing.push(handle); // Store the thread handle in the vector (return value of thread)
    }
    // Collect results from all threads and sum them up and then finally print the total count
    let mut total_count = 0;
    for handle in thread_storing {
        total_count += handle.join().unwrap();
    }
    println!("Finished counting 'the' and its total count is: {}", total_count);
}
