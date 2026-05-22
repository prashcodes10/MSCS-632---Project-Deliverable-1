# Simple Chat Application in Go and Rust

## Team Members
- Aisworya Maharjan
- Prashanna Acharya

## Course
Advanced Programming Languages (MSCS-632-A01)

---

# Project Overview

This project implements a simple text-based chat application using two programming languages:
- Go
- Rust

The purpose of this project is to compare how both programming languages implement similar application functionality while demonstrating language-specific features such as:
- Concurrency
- Memory Management
- Data Structures
- Error Handling
- Thread Communication

Both implementations provide the same core functionality while utilizing each language’s unique strengths.

---

# Core Features

The following features are implemented in both Go and Rust versions:

- Multiple users sending messages concurrently
- Message storage/history
- Sender and receiver tracking
- User ID tracking
- Message timestamps
- Keyword-based message searching
- User-based message searching
- Concurrent message handling
- Structured message storage using custom structs

---

# Project Structure

```text
simple-chat-app/
│
├── go-chat/
│   └── main.go
│
├── rust-chat/
│   └── main.rs
│
└── README.md
```

---

# Go Implementation

## Language-Specific Features Used

### Goroutines
Go uses goroutines to simulate multiple users sending messages concurrently.

### Channels
Channels are being used to safely pass messages between goroutines.

### Structs and Slices
Custom structs are storing messages while slices are storing message history.

---

# Rust Implementation

## Language-Specific Features Used

### Threads
Rust uses threads to simulate concurrent message handling.

### Channels
Rust channels are being used to send messages safely between threads.

### Ownership and Memory Safety
Rust’s ownership and borrowing model is providing memory safety without using garbage collection.

### Structs and Vectors
Custom structs are storing messages while vectors are storing message history.

---

# How to Run the Go Program

## Step 1
Install Go:

https://go.dev/dl/

## Step 2
Open terminal inside the `go-chat` folder.

## Step 3
Run the following command:

```bash
go run main.go
```

---

# How to Run the Rust Program

## Step 1
Install Rust:

https://www.rust-lang.org/tools/install

## Step 2
Open terminal inside the `rust-chat` folder.

## Step 3
Compile the Rust program:

```bash
rustc main.rs
```

## Step 4
Run the executable:

### Windows
```powershell
.\main.exe
```

### Mac/Linux
```bash
./main
```

---

# Go vs Rust Comparison

| Feature | Go | Rust |
|---|---|---|
| Concurrency | Goroutines and Channels | Threads and Channels |
| Memory Management | Garbage Collection | Ownership and Borrowing |
| Syntax | Simple and Minimal | Strict and Detailed |
| Performance | Fast | Very Fast |
| Safety | Good | Excellent Memory Safety |
| Learning Curve | Easier | More Complex |

---

# Learning Outcomes

Through this project, we learned:
- How different programming languages implement concurrency
- Differences between garbage collection and ownership-based memory management
- How Go simplifies concurrent programming
- How Rust enforces memory safety and thread safety
- How similar applications can be implemented differently depending on language design

---

# References

- Go Documentation  
  https://go.dev/doc/

- Rust Documentation  
  https://www.rust-lang.org/learn

- The Rust Programming Language Book  
  https://doc.rust-lang.org/book/

- Concepts of Programming Languages (Sebesta, 2019)

