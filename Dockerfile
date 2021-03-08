FROM rustlang/rust:nightly

RUN apt-get update
RUN apt-get install make -y

WORKDIR /code

COPY . .

CMD ["make", "start"]
