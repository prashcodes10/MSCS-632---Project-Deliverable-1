use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// Message is storing all information related to one chat message.
#[derive(Clone, Debug)]
struct Message {
    sender_id: u32,
    sender: String,
    receiver: String,
    text: String,
    timestamp: String,
}

// get_timestamp is creating a simple timestamp using the current system time.
fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    format!("Unix Timestamp: {}", now.as_secs())
}

// send_message is creating a message and sending it through a channel.
fn send_message(
    sender_id: u32,
    sender: &str,
    receiver: &str,
    text: &str,
    tx: mpsc::Sender<Message>,
) {
    // Creating a new message with sender, receiver, text, and timestamp.
    let message = Message {
        sender_id,
        sender: sender.to_string(),
        receiver: receiver.to_string(),
        text: text.to_string(),
        timestamp: get_timestamp(),
    };

    // Sending the message to the main thread through the channel.
    tx.send(message).expect("Failed to send message");
}

// display_messages is printing all stored messages.
fn display_messages(messages: &Vec<Message>) {
    println!("All Messages");
    println!("-----------------------------");

    // Looping through each stored message and displaying its details.
    for message in messages {
        println!("Sender ID: {}", message.sender_id);
        println!("Sender: {}", message.sender);
        println!("Receiver: {}", message.receiver);
        println!("Message: {}", message.text);
        println!("Time: {}", message.timestamp);
        println!("-----------------------------");
    }
}

// search_by_keyword is searching messages that contain a specific keyword.
fn search_by_keyword(messages: &Vec<Message>, keyword: &str) {
    println!("Search by keyword: {}", keyword);
    println!("-----------------------------");

    // Looping through each message and checking whether the text contains the keyword.
    for message in messages {
        if message.text.to_lowercase().contains(&keyword.to_lowercase()) {
            println!("Sender ID: {}", message.sender_id);
            println!("Sender: {}", message.sender);
            println!("Receiver: {}", message.receiver);
            println!("Message: {}", message.text);
            println!("Time: {}", message.timestamp);
            println!("-----------------------------");
        }
    }
}

// search_by_user is searching messages where the user is either the sender or receiver.
fn search_by_user(messages: &Vec<Message>, user: &str) {
    println!("Search by user: {}", user);
    println!("-----------------------------");

    // Looping through each message and checking whether the user is involved.
    for message in messages {
        if message.sender.eq_ignore_ascii_case(user)
            || message.receiver.eq_ignore_ascii_case(user)
        {
            println!("Sender ID: {}", message.sender_id);
            println!("Sender: {}", message.sender);
            println!("Receiver: {}", message.receiver);
            println!("Message: {}", message.text);
            println!("Time: {}", message.timestamp);
            println!("-----------------------------");
        }
    }
}

fn main() {
    // Creating a channel for sending Message objects between threads.
    let (tx, rx) = mpsc::channel();

    // Cloning the transmitter so multiple threads can send messages.
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        // Simulating Alice sending a message to Bob.
        send_message(1, "Alice", "Bob", "Hello Bob!", tx1);
    });

    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        // Simulating Bob sending a message to Alice.
        send_message(2, "Bob", "Alice", "Hi Alice!", tx2);
    });

    let tx3 = tx.clone();
    let handle3 = thread::spawn(move || {
        // Simulating Charlie sending a message to Alice.
        send_message(3, "Charlie", "Alice", "How are you?", tx3);
    });

    // Dropping the original transmitter so the receiver knows when sending is complete.
    drop(tx);

    // Waiting for all threads to finish sending their messages.
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    // Creating a vector to store message history.
    let mut messages: Vec<Message> = Vec::new();

    // Receiving all messages from the channel and storing them in message history.
    for message in rx {
        messages.push(message);
    }

    // Displaying all stored messages.
    display_messages(&messages);

    // Searching messages by keyword.
    search_by_keyword(&messages, "hello");

    // Searching messages by user.
    search_by_user(&messages, "Alice");
}