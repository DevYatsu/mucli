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
    CYGWIN*)    INSTALL_FILE="./mucli_windows.exe" ;;  # Cygwin (Windows-like environment)
    MINGW*)     INSTALL_FILE="./mucli_windows.exe" ;;  # MinGW (Windows-like environment)
    *)
        echo "Unsupported operating system."
        exit 1
        ;;
esac

cp $INSTALL_FILE $INSTALL_DIR/mucli

if [ $? -eq 0 ]; then
    # Provide executable permissions to "mucli" (not needed on Windows)
    if [ "$(uname -s)" != "MINGW"* ]; then
        chmod +x $INSTALL_DIR/mucli
    fi

    echo "mucli has been set up successfully!"
else
    echo "Installation failed. Please make sure you have the necessary permissions."
fi