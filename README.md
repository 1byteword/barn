# Barn
                                               
<p align="center">
  <img src="misc/barn.png" alt="Barn Logo" />
</p>

## About

Barn is a sophisticated command-line interface (CLI) tool that excels in managing, tokenizing, and securely encrypting data through a RESTful API implemented in Rust. Barn offers an advanced cryptographic solution — more sophisticated and intuitive than current industry leaders — that ensures data confidentiality and integrity. This tool was developed to offer a dependable alternative in the wake of significant industry consolidations, such as IBM's acquisition of HashiCorp, reinforcing the necessity for resilient and independent data security solutions. Barn enables users to encrypt data and securely store it, facilitating the retrieval of the original data using unique keys, thus ensuring the security of data across platforms. Welcome to the Barnyard.

## Installation

You can install Barn using Homebrew. Follow these steps:

1. **Tap the Repository**:
    ```sh
    brew tap 1byteword/barn https://github.com/1byteword/homebrew-tap
    ```

2. **Install Barn**:
    ```sh
    brew install barn
    ```

## Usage

<img src="misc/home.png" alt="Barnyard Home" />

### Start the Server

To start the Barn server, run:
```sh
./barn serve --address 127.0.0.1:8000
```

### Encrypt and Store Data

To securely store data with encryption, use the following curl command:

```bash
curl -X POST http://127.0.0.1:8000/store -H 'Content-Type: application/json' -d '{\"key\": \"exampleKey\", \"value\": \"exampleValue\"}'
```

This command encrypts the value and stores it under the specified key.

### Decrypt and Retrieve Data

To retrieve and decrypt data, use the following curl command:

```bash
curl -X POST http://127.0.0.1:8000/load -H 'Content-Type: application/json' -d '{\"key\": \"exampleKey\"}'
```

This retrieves the encrypted data using the specified key and decrypts it.

### Example Workflow

1. **Start the Server**:
    ```bash
    ./barn serve --address 127.0.0.1:8000
    ```

2. **Encrypt and Store Data**:
    ```bash
    curl -X POST http://127.0.0.1:8000/store -H 'Content-Type: application/json' -d '{"key": "exampleKey", "value": "exampleValue"}'
    ```

    Example response:
    ```json
    {
      "message": "Key-value pair stored successfully"
    }
    ```

3. **Decrypt and Retrieve Data**:
    ```bash
    curl -X POST http://127.0.0.1:8000/load -H 'Content-Type: application/json' -d '{"key": "exampleKey"}'
    ```

    Example response:
    ```json
    {
      "value": "exampleValue"
    }
    ```

### Viewing Logs

For detailed logs of server activities, including encryption and decryption operations, set the `RUST_LOG` environment variable to `info` before starting the server:

```bash
RUST_LOG=info ./barn serve --address 127.0.0.1:8000
```