# Build Dependencies

This project requires the following system dependencies to build successfully:

## Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libglib2.0-dev \
    libgobject-2.0-0 \
    pkg-config
```

## macOS

The project should build without additional dependencies on macOS.

## Windows

The project should build without additional dependencies on Windows.

## Why these dependencies?

- **libgtk-3-dev**: Required by Tauri for the GUI framework
- **libglib2.0-dev**: Core library used by GTK
- **libgobject-2.0-0**: Object system library used by GTK
- **pkg-config**: Used to locate and configure the above libraries during build

## CI/CD

These dependencies are automatically installed in our GitHub Actions workflows for Linux builds.