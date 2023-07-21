# mucli (Multi Use Command Line Interface)

mucli is a versatile command-line tool offering various functionalities, including file encryption/decryption, secure password storage, and more. It provides a user-friendly interface to perform different tasks efficiently.

## Installation

To use mucli, follow these steps:

- [Download the latest](../../releases) release for your operating system.
- Unzip the downloaded file to a preferred location on your machine.
- Navigate to the unzipped folder in your terminal or command prompt.

## Usage

mucli supports several commands and options to cater to your needs. Here are the available commands:

### Set Password

Command to set a security password to access sensitive data.

`mucli password --flag`

```bash
# Set a password for the first time
mucli password -i [optional new_password]

# Change the existing password
mucli password --change [optional current_password]

# Reset the password by answering a set of questions (future release)
mucli password --reset

# Add and remove questions you will have to answer to reset your password
mucli password --modifyQ [optional current_password]
```

### Encrypt

Command to encrypt a specified file and place the output file in the specified directory.

```bash
# Encrypt the file and place the output in the specified directory
mucli encrypt /path/to/source_file /path/to/output_dir

# Encrypt the file and place the output in the current directory
mucli encrypt -c /path/to/source_file

# Update the encryption key of the file
mucli encrypt -u /path/to/source_file

# Update encryption key version
mucli encrypt -u

# Replace the file by its encrypted version
mucli encrypt -s /path/to/source_file

# Encrypt the file 5 times
mucli encrypt -t 5 /path/to/source_file
```

### Decrypt

Command to decrypt a specified file and place the output file in the specified directory.

```bash
# Decrypt the file and place the output in the specified directory
mucli decrypt /path/to/encrypted_file /path/to/output_dir

# Decrypt the file and place the output in the current directory
mucli decrypt -c /path/to/encrypted_file

# Replace the file by its decrypted version
mucli decrypt -s /path/to/encrypted_file

# Decrypt the target file until it's totally decrypted,
# useful when crypted several times
mucli decrypt -e /path/to/encrypted_file
```

## Feedback and Contributions

We welcome your feedback and contributions to improve mucli. If you encounter any issues or have suggestions for new features, please feel free to open an [issue](../../issues) on our GitHub repository.

Happy command-line multitasking with mucli!
