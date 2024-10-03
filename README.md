# Zoomark

Zoomark is a command-line tool that lets you bookmark and manage frequently used commands, helping you quickly access and execute them when needed.

## Features

- Save bookmarks for any command with a name and description.
- Easily list all saved bookmarks.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/zoomark.git
   cd zoomark
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Install Zoomark globally:

   ```bash
   cargo install --path .
   ```

## Usage

### Add a bookmark

To save a new command bookmark:

```bash
zoomark -c "<command_name>"  -d "<command_description>" 
```

Example:

```bash
zoomark -c "ls"  -d "Lists all files in the home directory" 
```

### List all bookmarks

To list all saved bookmarks:

```bash
zoomark -s
```


## Configuration

Zoomark saves bookmarks in a configuration file located at:

`~/.config/zoomark/command_user.json`

This file can be manually edited if necessary, but it's recommended to use the app's commands for consistency.
