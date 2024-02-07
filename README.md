# ChatGPT Console

This is a simple console-based application that interacts with the OpenAI ChatGPT model. It allows users to input text and receive responses generated by the ChatGPT model.

## Technology Stack Used

- **Rust**: Programming language used to develop the application.
- **Reqwest**: HTTP client for making requests to the OpenAI API.
- **Serde**: Serialization and deserialization library for JSON data.
- **Anyhow**: Library for handling errors with less boilerplate code.
- **Tokio**: Asynchronous runtime for Rust.

## How to Set Up and Use

1. **Clone the Repository**: Clone this repository to your local machine using the following command:

   ```bash
   git clone https://github.com/yourusername/chatgpt-console.git
   ```

2. **Install Rust**: If you haven't already, you'll need to install Rust. You can do this using Rust's official installer, `rustup`, available at [rustup.rs](https://rustup.rs/).

3. **Set Up OpenAI API Key**: You need an API key from OpenAI to use the ChatGPT API. If you don't have one, sign up for OpenAI and obtain your API key.

4. **Configure API Key**: Open the `main.rs` file in the project directory and replace the placeholder API key with your actual API key.

5. **Build and Run**: Run the following command in the project directory to build and run the application:

   ```bash
   cargo run
   ```

6. **Interact with ChatGPT**: Once the application is running, you can interact with ChatGPT by typing your messages in the console. Type `/exit` to quit the application.

7. **Customization**: You can customize the behavior of the application by modifying the code in the `main.rs` file. For example, you can adjust the maximum number of tokens generated by ChatGPT by changing the `max_tokens` parameter.

## Contributors

- Your Name (@yourgithubusername)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.