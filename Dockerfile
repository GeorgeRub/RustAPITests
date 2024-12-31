FROM ubuntu:24.04
COPY target/release/tone .
COPY .env .
EXPOSE 8000/tcp
CMD ["./tone"]