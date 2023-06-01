FROM golang:latest

RUN mkdir -p /home/app
#Compilation daemon. this helps with development in docker
RUN go install github.com/githubnemo/CompileDaemon@latest
RUN mv /go/bin/CompileDaemon /usr/bin

COPY . /home/app

WORKDIR /home/app/

EXPOSE 3000

#CMD ["go", "run", "." ]
CMD ["CompileDaemon", "-command=./main" ]
