# mucli (Multi Use Command Line Interface)

mucli is a versatile command-line tool offering various functionalities, including file encryption/decryption, secure password storage, and more. It provides a user-friendly interface to perform different tasks efficiently.

## Installation

To use mucli, follow these steps:

- [Download the latest](../../releases) release for your operating system.
- Unzip the downloaded file to a preferred location on your machine.
- Navigate to the unzipped folder in your terminal or command prompt.

## Usage

mucli supports several commands and options to cater to your needs. Commands can sometimes need admin access to be executed. Here are the available commands:

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

### Rename

Command to rename a file.

```bash
mucli rename [FILEPATH] [NAME]
```

### Copy

Command to copy a file's content into another existing or non-existing file or into a directory.

```bash
mucli copy [FILEPATH] [TARGET]
```

### Move

Command to move a file into a directory.

```bash
mucli move [FILEPATH] [DIR]
```

### Zip

Command to compress the specified file/folder and place the output zip file in the specified directory.

```bash
# Compress the file/folder and place the output in the same directory as the source
mucli zip [PATH]

# Compress the file/folder and place the output in the specified directory
mucli zip [PATH] [OUTPUTDIR]

# Compress the file/folder and place the output in the current directory
mucli zip -c [PATH]

# Compress with a specified compression level (0 to 9)
mucli zip -l [LEVEL] [PATH]
```

### Unzip

Command to extract the specified zip file and place the output file/folder in the specified directory.

```bash
# Compress the file/folder and place the output in the same directory as the source
mucli zip [PATH]

# Compress the file/folder and place the output in the specified directory
mucli zip [PATH] [OUTPUTDIR]

# Compress the file/folder and place the output in the current directory
mucli zip -c [PATH]
```

### Shell

Command to execute a shell script.

```bash
mucli shell [FILEPATH]
```

### Antivirus

Command to check for malwares in a given file, using the virustotal API.

```bash
mucli antivirus [FILEPATH]
```

The `antivirus` command returns:

- Malicious threats detected: Number of engine reports indicating that the file is malicious.
- Suspicious threats detected: Number of engine reports indicating that the file is suspicious.

### Timer

Command to use a simple timer.

```bash
mucli timer
```

### Update (still in development)

Command to check if a new update of mucli is available (coming soon).

```bash
mucli update
```

## Ideas of commands to implement in the future

- "extract": Create a command to extract files from compressed archives, supporting popular formats like ZIP, TAR, GZ, etc.
- "history": Implement a command to show the command history, allowing users to recall and re-execute previous commands.
- "timer": Develop a command-line timer or stopwatch.
- "network": Implement a command to display network-related information, such as IP address, network interfaces, and connectivity status.
- "processes": Create a command to view running processes and manage them, allowing users to terminate or prioritize tasks.
- "system-info": Develop a command to provide essential system information, including CPU, memory, disk usage, and OS details.
- "currency": Create a command to convert between different currencies, using up-to-date exchange rates.
- "dictionary": Develop a command-line dictionary tool to look up word definitions, synonyms, and antonyms.
- "translator": Implement a command to translate text between different languages using popular translation APIs.
- "random": Create a command to generate random numbers, passwords, or strings.
- "calculator": Implement a command-line calculator capable of performing basic arithmetic and advanced mathematical operations.
- "qr-code": Develop a command to generate QR codes for text or URLs, as well as decode existing QR codes.
- "weather": Implement a command to display weather forecasts and current conditions for a specified location.

## Feedback and Contributions

We welcome your feedback and contributions to improve mucli. If you encounter any issues or have suggestions for new features, please feel free to open an [issue](../../issues) on our GitHub repository.

Happy command-line multitasking with mucli!
