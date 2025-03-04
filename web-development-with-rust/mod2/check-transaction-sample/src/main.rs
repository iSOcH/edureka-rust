use std::sync::atomic::{Ordering, AtomicUsize};
use std::{thread, time::Duration};

// This is modified code from https://www.coursera.org/learn/web-development-with-rust/lecture/3WsWf/transaction-handling-in-rust to check
// whether the statement made ("This example demonstrates atomicity. The balanced check and update happen in a single operation.") is correct

const STARTING_BALANCE: usize = 100;
static BALANCE: AtomicUsize = AtomicUsize::new(STARTING_BALANCE);

fn main() {
    let withdraw_slow = thread::spawn(|| withdraw(60, Duration::from_millis(1000)));
    
    // give some time for withdraw_slow to read
    thread::sleep(Duration::from_millis(50));

    withdraw(60, Duration::ZERO).unwrap();
    let balance_after_first = BALANCE.load(Ordering::SeqCst);
    println!("Balance after first: {balance_after_first}");
    assert_eq!(STARTING_BALANCE - 60, balance_after_first);

    _ = withdraw_slow.join().unwrap().unwrap(); // join() returns Result<Result<...>>

    let balance_after = BALANCE.load(Ordering::SeqCst);
    println!("Balance after both: {balance_after}");
    assert!(balance_after < STARTING_BALANCE); // second deduction underflows, causing this assertion to be violated
}

fn withdraw(amount: usize, delay: Duration) -> Result<(), String> {
    let current_balance = BALANCE.load(Ordering::SeqCst);
    println!("Thread {:?}: read {current_balance} at start", thread::current().id());

    if current_balance >= amount {
        println!("Thread {:?}: will deduct...", thread::current().id());
        thread::sleep(delay);
        BALANCE.fetch_sub(amount, Ordering::SeqCst);
        println!("Thread {:?}: deducted...", thread::current().id());
        Ok(())
    } else {
        Err("insufficient funds".into())
    }
}