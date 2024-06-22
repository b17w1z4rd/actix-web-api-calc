# Actix Web API with Rhai for Arithmetic Operations

This project is a simple web API built with [Actix Web](https://actix.rs/) and [Rhai](https://rhai.rs/). The API provides endpoints to perform basic arithmetic operations (multiplication and addition) by evaluating Rhai scripts.

## Features

- **Multiplication Endpoint**: Multiplies two numbers.
- **Addition Endpoint**: Adds two numbers.
- **Rhai Script Evaluation**: Uses Rhai scripting engine to execute arithmetic operations.

## Endpoints

1. **Multiply**
    - **URL**: `/multiply/{num1}/{num2}`
    - **Method**: GET
    - **Description**: Multiplies `num1` and `num2` and returns the result.
    - **Example**: 
        ```sh
        curl http://127.0.0.1:8080/multiply/3/4
        ```
      - **Response**: `12`

2. **Add**
    - **URL**: `/add/{num1}/{num2}`
    - **Method**: GET
    - **Description**: Adds `num1` and `num2` and returns the result.
    - **Example**:
        ```sh
        curl http://127.0.0.1:8080/add/3/4
        ```
      - **Response**: `7`

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (Nightly)
- Actix Web
- Rhai

## Setup and Running

1. **Clone the repository**:
    ```sh
    https://github.com/b17w1z4rd/actix-web-api-calculator.git
    cd actix-web-api-calculator
    ```

2. **Add dependencies**: Edit `Cargo.toml` to include `actix-web` and `rhai`.
    ```toml
    [dependencies]
    actix-web = "4"
    rhai = "1.8"
    ```

3. **Create Rhai script files**:

    - `multiply.rhai`:
        ```rhai
        fn multiply() {
            num1() * num2()
        }
        multiply()
        ```

    - `add.rhai`:
        ```rhai
        fn add() {
            num1() + num2()
        }
        add()
        ```

4. **Run the server**:
    ```sh
    cargo run
    ```

5. **Access the endpoints**:
    - Multiply: `http://127.0.0.1:8080/multiply/{num1}/{num2}`
    - Add: `http://127.0.0.1:8080/add/{num1}/{num2}`

## Example

To test the API, you can use tools like `curl`:

- **Multiplication**:
    ```sh
    curl http://127.0.0.1:8080/multiply/3/4
    ```

- **Addition**:
    ```sh
    curl http://127.0.0.1:8080/add/3/4
    ```

## License

This project is licensed under the MIT License.
