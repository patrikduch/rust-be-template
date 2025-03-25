# Overview

`rust-be-template` is a boilerplate template for a backend service written in Rust. It provides a solid foundation with essential dependencies and configurations to streamline backend development.

## Features

- **Actix-Web** for building RESTful APIs


## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/patrikduch/rust-be-template.git
cd rust-be-template
```

### 2. Set Up Environment Variables

Copy the example `.env.example` file and configure it:

```sh
cp .env.example .env
```

Modify the `.env` file with your database credentials and other configurations.


### 3. Build and Run the Application

```sh
cargo build
cargo run
```


### 4. Dockerization

docker build -t rust-be-template .


docker run  --name rust-be-container -p 8080:8080 --env RUST_LOG=debug rust-be-template



docker-compose up -d


docker build -t rust-be-template  .

docker tag rust-be-template:latest patrikduch/rust-be-template:latest

docker push patrikduch/rust-be-template:latest


