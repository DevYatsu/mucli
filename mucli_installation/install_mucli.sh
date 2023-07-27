#!/bin/bash

# Define the installation directory based on the operating system
case "$(uname -s)" in
    Linux*)     INSTALL_DIR="/usr/local/bin" ;;  # Linux
    Darwin*)    INSTALL_DIR="/usr/local/bin" ;;  # macOS
    CYGWIN*)    INSTALL_DIR="/usr/bin"       ;;  # Cygwin (Windows-like environment)
    MINGW*)     INSTALL_DIR="/usr/bin"       ;;  # MinGW (Windows-like environment)
    *)
        echo "Unsupported operating system."
        exit 1
        ;;
esac

# Define the installation file based on the operating system
case "$(uname -s)" in
    Linux*)     INSTALL_FILE="./mucli_linux" ;;  # Linux
    Darwin*)    INSTALL_FILE="./mucli_macos" ;;  # macOS
    CYGWIN*)    INSTALL_FILE="./mucli_windows" ;;  # Cygwin (Windows-like environment)
    MINGW*)     INSTALL_FILE="./mucli_windows" ;;  # MinGW (Windows-like environment)
    *)
        echo "Unsupported operating system."
        exit 1
        ;;
esac

# Copy "mucli" to the installation directory
cp $INSTALL_FILE $INSTALL_DIR/mucli

# Check if the copy command was successful
if [ $? -eq 0 ]; then
    # Provide executable permissions to "mucli" (not needed on Windows)
    if [ "$(uname -s)" != "MINGW"* ]; then
        chmod +x $INSTALL_DIR/mucli
    fi

    # Inform the user that the installation is complete
    echo "mucli has been set up successfully!"
else
    echo "Installation failed. Please make sure you have the necessary permissions."
fi