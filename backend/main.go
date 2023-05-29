
package main

import (
	"log"
	"net/http"
	"os"
	"fmt"

	"github.com/gin-gonic/gin"
)


func main(){
	router:= gin.Default()

	router.GET("/",index)
	router.GET("api/getfiles",getFiles) // returns a json with all the files that we can deliver
	router.GET("api/redirdoc",getDoc) // sends the html file
	fmt.Println("starting")
	router.Run("localhost:3000")
}

func index(c *gin.Context){
	c.File("../homepage/index.html")
}

//what is returned to the client when they ask for filenames
type entry struct{
	Name string `json:"name"`
}

func getFiles(c *gin.Context){
	c.Header("Access-Control-Allow-Origin", "*")
	entries,err:=os.ReadDir("../documents")
	if err!=nil{
		log.Fatal(err)
	}

	//idk if i can make this into a one liner
	var things = make([]entry, len(entries))
	for i,e:= range entries{
		things[i]=entry{Name:e.Name()}
	}


	c.JSON(http.StatusOK, things)

}

func getDoc(c *gin.Context){
	name:=c.Query("name") //this is how to get fields from the request i think
	fmt.Println(name)
	c.File("../documents/"+name)
}
