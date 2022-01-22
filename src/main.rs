use std::marker::Send;
use std::thread;
use std::sync::{mpsc, mpsc::Sender, mpsc::Receiver};

type Function<T, R> = fn(t: T) -> R;
type InputPair<T> = (usize, T);
type OutputPair<R> = (usize, R);

// Treshold
const THRESHOLD: i64 = 8;

// Maximum number of threads
const MAX_NUM_OF_THREADS: i64 = 64;

// Splitting of computational work
fn split_comp_work<T, R>(vector: Vec<T>, function: Function<T, R>) -> Vec<R>
    where T: 'static + Clone + Send, R: 'static + Default + Clone + Send {
    
    // If length of the vector less than the threshold then no threads are needed
    if vector.len() < THRESHOLD as usize {
        
        // Printing debugging information
        println!("Doing computational work in the current thread");
        
        return do_comp_work_in_cur_thread(vector, function);
    }
    
    // Channel for transferring results of computational work
    let (sender, receiver): (Sender<InputPair<R>>, Receiver<OutputPair<R>>) = mpsc::channel();
    
    let mut index_of_cur_item: usize = 0;
    
    // Number of threads
    let mut num_of_threads: i64 = ((vector.len() as f64) / (THRESHOLD as f64)).ceil() as i64;
    
    // Number of threads can't be more than maximum number of threads
    if num_of_threads > MAX_NUM_OF_THREADS {
        num_of_threads = MAX_NUM_OF_THREADS;
    }
    
    // Number of items per one thread
    let items_per_thread: i64 = ((vector.len() as f64) / (num_of_threads as f64)).ceil() as i64;
    
    // Spawning threads for computational work
    for i in 0..num_of_threads {
        
        // Creating copy of slice of the vector
        let mut end_index = index_of_cur_item + (items_per_thread as usize);
        
        if end_index > vector.len() {
            end_index = vector.len();
        }
        
        let mut vector_copy: Vec<InputPair<T>> = Vec::new();
        vector_copy.reserve(end_index - index_of_cur_item);
        
        for (index, item) in vector[index_of_cur_item..end_index].iter().enumerate() {
            vector_copy.push((index_of_cur_item + index, item.clone()));
        }
        
        let sender_copy = sender.clone();
        
        thread::spawn(move || {
            do_comp_work_in_some_thread(vector_copy, sender_copy, function)
        });
        
        index_of_cur_item = end_index;
        
        // Printing debugging information
        println!("Thread {} has spawned", i);
    }
    
    // Releasing the first non-used sender
    drop(sender);
    
    let mut result: Vec<R> = Vec::new();
    result.resize(vector.len(), Default::default());
    
    // Receiving results
    for received in receiver {
        let (index, item) = received;
        result[index] = item;
    }
    
    result
}

// Doing computational work in current thread
fn do_comp_work_in_cur_thread<T, R>(vector: Vec<T>, function: Function<T, R>) -> Vec<R> {
    let mut result: Vec<R> = Vec::new();
    result.reserve(vector.len());
    
    for item in vector {
        result.push(function(item));
    }
    
    result
}

// Doing computational work in some thread
fn do_comp_work_in_some_thread<T, R>(vector: Vec<InputPair<T>>, sender: Sender<OutputPair<R>>,
    function: Function<T, R>) {
    for (index, item) in vector {
        let result = (index, function(item));
        
        // Sending result
        sender.send(result).expect("Can't send result value by thread");
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// Base for checking number for evenness
const EVEN_BASE: i64 = 2;

// Example of client's function
fn is_even(num: i64) -> bool {
    num.checked_rem(EVEN_BASE).expect("Invalid number for checking for evenness") == 0
}

fn main() {
    test_a();
    test_b();
}

fn test_a() {
    
    // Printing debugging information
    println!("Starting computational work...");
    
    // Example of client's code
    let vector = vec![1, 2, 3, 4];
    
    let result = split_comp_work(vector, is_even);
    
    let result_for_check = vec![false, true, false, true];
    
    // Checking result
    assert_eq!(result, result_for_check);
    
    // Printing debugging information
    println!("Computational work has been completed");
}

fn test_b() {
    
    // Printing debugging information
    println!("Starting computational work...");
    
    // Example of client's code
    let vector = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        31, 32, 33, 34];
    
    let result = split_comp_work(vector, is_even);
    
    let result_for_check = vec![
        false, true, false, true, false, true, false, true, false, true,
        false, true, false, true, false, true, false, true, false, true,
        false, true, false, true, false, true, false, true, false, true,
        false, true, false, true];
    
    // Checking result
    assert_eq!(result, result_for_check);
    
    // Printing debugging information
    println!("Computational work has been completed");
}
