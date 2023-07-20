# mucli (Multi Use Command Line Interface)

mucli is a versatile command-line tool offering various functionalities, including file encryption/decryption, secure password storage, and more. It provides a user-friendly interface to perform different tasks efficiently.

## Installation

To use mucli, follow these steps:

- [Download the latest](./../releases/latest) release for your operating system.
- Unzip the downloaded file to a preferred location on your machine.
- Navigate to the unzipped folder in your terminal or command prompt.

## Usage

mucli supports several commands and options to cater to your needs. Here are the available commands:

### Set Password

Command to set a security password to access sensitive information.

`mucli password --flag`

#### Flags (one required):

- `-i, --init`: Set a password for the first time.
- `-c, --change`: Change the existing password.
- `-r, --reset` (future release): Reset the password.

#### Example usage:
```
# Set a password for the first time
mucli password -i [optional new_password]

# Change the existing password
mucli password --change [optional current_password]
```

### Encrypt

Command to encrypt a specified file and place the output file in the specified directory.

`mucli encrypt [options] [file_path] [output_dir]`

#### Flags (not required):

- `-u, --ukey`: Update encryption key or update the encryption key of a file to the latest version.
- `-c, --cdir`: Place the output file in the current directory.
- `-s, --sfile`: Select the target file as the output file.

#### Example usage:
```
# Encrypt the file and place the output in the current directory
mucli encrypt -c /path/to/source_file

# Update encryption key of the file
mucli encrypt -u /path/to/source_file

# Update encryption key version
mucli encrypt -u

# Replace file by its encrypted version
mucli encrypt -s /path/to/source_file
```

### Decrypt

Command to decrypt a specified file and place the output file in the specified directory.

`mucli decrypt [options] [file_path] [output_dir]`

#### Flags (not required):

- `-c, --cdir`: Place the output file in the current directory.
- `-s, --sfile`: Select the target file as the output file.

#### Example usage:
```
# Encrypt the file and place the output in the current directory
mucli decrypt -c /path/to/source_file

# Replace file by its encrypted version
mucli decrypt -s /path/to/source_file
```

## Feedback and Contributions

We welcome your feedback and contributions to improve mucli. If you encounter any issues or have suggestions for new features, please feel free to open an [issue](../issues) on our GitHub repository.

Happy command-line multitasking with mucli!
