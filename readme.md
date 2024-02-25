# Clippy - A Clipboard Synchronization Tool

![clippp.webp](assets/clippp.webp)

## What is Clippy?

Clippy is designed to synchronize clipboard contents across multiple instances running on a local network. It uses UDP broadcasting for discovery of other instances and establishes TCP connections for efficient clipboard data exchange. Whenever the clipboard content changes in one instance, it automatically updates the clipboard contents of all other discovered instances.

Clippy is built using the Rust programming language and the `clipboard` crate for clipboard access. It is designed to be lightweight, efficient, and easy to use.
However, this code is not yet ready for production use. It is still in the early stages of development and may contain bugs or incomplete features.

## How It Works

- **Discovery Phase:** Upon launch, each instance broadcasts a discovery message over the local network using UDP. This allows all instances of the application to become aware of each other. A heartbeat is set so to make any new instance aware of the preexisting instance.
- **Synchronization Phase:** After discovery, instances use TCP connections to synchronize clipboard contents. Any change in the clipboard of one instance is sent to all other instances, ensuring all clipboards are kept in sync.

## Getting Started

### Prerequisites

- Rust Programming Language: Ensure you have Rust installed on your machine. If not, you can download and install it from [the official Rust website](https://www.rust-lang.org/tools/install).
- Clipboard Access: This application requires access to the system's clipboard, which is managed through the `clipboard` crate.

### Installation

1. **Clone the Repository**

   First, clone the repository to your local machine using Git:

   ```sh
   git clone https://github.com/kod3000/clippy.git
   cd clippy
   ```


2. **Build/Run the Application**

   To build and run the application, use the following command:

   ```sh
   cargo run
   ```

   This will build and run the application on your local machine.


### Usage

Once the application is running, it will automatically start broadcasting discovery messages and listening for other instances on the local network.
You should only run one instance of the application on each machine you'd like to keep synchronized.

When you copy text to the clipboard in one instance, it will automatically update the clipboard contents of all other instances. You can test this by copying text in one instance and then pasting it into another instance.


## Developer Notes

- Each instance uses port 7878 for TCP connections.
- Since we set the ip address to 0.0.0.0, it will listen to all the interfaces on the machine.
- Using all interfaces means that the app will register itself as a connection point (bug)

The origins of this project stem from an initial creation of separate client and server versions. 
I later recognized the need for a unified application capable of acting both as a server and a client 
simultaneously, utilizing UDP for discovery and TCP for data exchange.

My future plans include incorporating features such as encryption, multi-file support, and support for distributed network systems. However, for this version, simplicity is key. I'm releasing this project to share my learning journey with others and hope you find it beneficial.

## Contributing

Contributions are very welcome! If you would like to contribute to this project, please feel free to open a pull request or submit an issue. I am always open to new ideas and improvements.


## Troubleshooting

Hmmm... if you are having trouble, I'm always happy to help anyway I can...
Please open an issue and I'll address it in a timely manner.


## Acknowledgments

Please throw a Star if you find this repo helpful, it's nice to get recognition for the work that goes into these projects. Thank you and Happy Coding!!!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
