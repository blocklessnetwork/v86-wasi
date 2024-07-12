package main

import (
	"log"
	"net/http"
)

func main() {

	staticPath := "."

	http.Handle("/", http.FileServer(http.Dir(staticPath)))

	log.Println("Listening...")
	err := http.ListenAndServe(":8081", nil)
	if err != nil {
		log.Fatal("ListenAndServe: ", err)
	}
}
