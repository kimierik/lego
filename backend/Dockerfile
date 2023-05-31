FROM golang:latest

RUN mkdir -p /home/app




#copy only needed files
COPY . /home/app




WORKDIR /home/app/

EXPOSE 3000

CMD ["go", "run", "."]

