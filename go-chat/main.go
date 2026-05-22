package main

import (
	"fmt"
	"strings"
	"time"
)

// Message is storing all information related to one chat message.
type Message struct {
	SenderID  int
	Sender    string
	Receiver  string
	Text      string
	Timestamp time.Time
}

// sendMessage is creating a message and sending it through the channel.
func sendMessage(senderID int, sender string, receiver string, text string, chatChannel chan Message) {
	// Creating a new message with sender, receiver, text, and current time.
	message := Message{
		SenderID:  senderID,
		Sender:    sender,
		Receiver:  receiver,
		Text:      text,
		Timestamp: time.Now(),
	}

	// Sending the message into the chat channel.
	chatChannel <- message
}

// displayMessages is printing all stored messages.
func displayMessages(messages []Message) {
	fmt.Println("All Messages")
	fmt.Println("-----------------------------")

	// Looping through each stored message and displaying its details.
	for _, message := range messages {
		fmt.Println("Sender ID:", message.SenderID)
		fmt.Println("Sender:", message.Sender)
		fmt.Println("Receiver:", message.Receiver)
		fmt.Println("Message:", message.Text)
		fmt.Println("Time:", message.Timestamp.Format("2006-01-02 15:04:05"))
		fmt.Println("-----------------------------")
	}
}

// searchByKeyword is searching messages that contain a specific keyword.
func searchByKeyword(messages []Message, keyword string) {
	fmt.Println("Search by keyword:", keyword)
	fmt.Println("-----------------------------")

	// Looping through each message and checking whether the text contains the keyword.
	for _, message := range messages {
		if strings.Contains(strings.ToLower(message.Text), strings.ToLower(keyword)) {
			fmt.Println("Sender ID:", message.SenderID)
			fmt.Println("Sender:", message.Sender)
			fmt.Println("Receiver:", message.Receiver)
			fmt.Println("Message:", message.Text)
			fmt.Println("Time:", message.Timestamp.Format("2006-01-02 15:04:05"))
			fmt.Println("-----------------------------")
		}
	}
}

// searchByUser is searching messages where the user is either the sender or receiver.
func searchByUser(messages []Message, user string) {
	fmt.Println("Search by user:", user)
	fmt.Println("-----------------------------")

	// Looping through each message and checking whether the user is involved.
	for _, message := range messages {
		if strings.EqualFold(message.Sender, user) || strings.EqualFold(message.Receiver, user) {
			fmt.Println("Sender ID:", message.SenderID)
			fmt.Println("Sender:", message.Sender)
			fmt.Println("Receiver:", message.Receiver)
			fmt.Println("Message:", message.Text)
			fmt.Println("Time:", message.Timestamp.Format("2006-01-02 15:04:05"))
			fmt.Println("-----------------------------")
		}
	}
}

func main() {
	// Creating a channel for sending Message objects between goroutines.
	chatChannel := make(chan Message)

	// Starting multiple goroutines to simulate different users sending messages at the same time.
	go sendMessage(1, "Alice", "Bob", "Hello Bob!", chatChannel)
	go sendMessage(2, "Bob", "Alice", "Hi Alice!", chatChannel)
	go sendMessage(3, "Charlie", "Alice", "How are you?", chatChannel)

	// Creating a slice to store message history.
	var messages []Message

	// Receiving three messages from the channel and storing them in message history.
	for i := 0; i < 3; i++ {
		message := <-chatChannel
		messages = append(messages, message)
	}

	// Displaying all stored messages.
	displayMessages(messages)

	// Searching messages by keyword.
	searchByKeyword(messages, "hello")

	// Searching messages by user.
	searchByUser(messages, "Alice")
}