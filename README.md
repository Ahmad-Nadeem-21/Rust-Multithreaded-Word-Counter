# Rust-Multithreaded-Word-Counter
This is a simple program that detects how many times a chosen word appears in a given sentence.
  
# How it works:
3 inputs:
1. A sentence
2. The number of threads to be spawned
3. The word to be counted
- The program splits the sentence into chunks based on the number of threads.
- Each thread counts the number of occurences of the chosen word in its own chunk.
- The threads return their local counts which is then totaled and displayed.
  
# Running the program:  
Format:  
cargo run -- (sentence) (numThreads) (word)  
  
Example:  
cargo run -- the fish in the sea are many, the fish 5 the  
  
Output example:  
Enter word to count number of chose word in a phrase  
Count in chunk: 1  
Count in chunk: 1  
Count in chunk: 0  
Count in chunk: 1  
Count in chunk: 0  
Finished counting 'the' and its total count is: 3  
  
# Notes  
If the number of threads is invalid it defaults to 4.  
The sentence is split based on spaces.  
  
Each thread is processed independently and returned so your output may differ from example output  

