# PROJECT: REAL TIME CHAT SERVER IN RUST
## DESCRIPTION
I create this project to enhance `Rust` skills and level up project management.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Installation

To get a local copy up and running follow these simple steps.

### Prerequisites
Make sure you have `Rust` installed by run this command:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Usage
Below are some examples of how you can use the project.

1. Clone this repository:
```sh 
git clone git@github.com:sangkhuudev/chat-server-rust.git
```

2. Backend uses `rocket framework`.
To run the backend server (at port 8000) :
  ```sh
cd backend
cargo run
  ```
3. Frontend uses `Yew framework`.
Open another terminal and run the client (at port 8080) :
```sh
cd frontend
trunk serve
```
4. Accessing the Chat Server
Head to `http://localhost:8080` in your web browser to access the chat server and enjoy chatting.

## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.
1. Fork the Project.
2. Create your Feature Branch (git checkout -b feature/AmazingFeature).
3. Commit your Changes (git commit -m 'Add some AmazingFeature').
4. Push to the Branch (git push origin feature/AmazingFeature).
5. Open a Pull Request.

## License
Distributed under the MIT License. See LICENSE for more information.

## Acknowledgements

- [Rocket Framework](https://rocket.rs/)
- [Yew Framework](https://yew.rs/)
- [Rust Programming Language](https://www.rust-lang.org/)
