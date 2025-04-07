# Wifirst AutoLogin

This is a Rust application that automates logging into the Planet Campus Wifirst network using Playwright and
configuration from a JSON file.

## Features

- **Automated Login**: The script automatically fills in the email and password fields and clicks the submit button to
  log in.
- **Configuration Management**: Configuration is managed via a `config.json` file, which should be placed in the same
  directory as the executable.

## Build

1. **Clone the repository**:
   ```sh
   git clone https://github.com/chianti-ga/wifirst-autologin.git
   cd wifirst-autologin
   ```

2. **Build**:
   ```sh
   cargo build --release
   ```

3. **Create a configuration file** (`config.json`) in the same directory as the executable, with your email and
   password:
   ```json
   {
       "email": "your_email@example.com",
       "password": "your_password"
   }
   ```