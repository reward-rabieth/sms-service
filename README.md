# 📱 SMS Service
An interactive SMS service for managing speaker assignments with automatic notifications and commands.


# 🎯 Project Overview
This project is a speaker management system that integrates with Twilio for SMS communications. When a speaker is added to the database, they receive an SMS asking if they accept or decline the assignment. Their response is then forwarded to the administrator. Speakers can also use text commands to receive information about their assignments.
 
# ✨ Features
-  [x]  CLI interface for adding speakers with name, phone, and optional date

-  [x]   SMS notification to speakers when added to the system

-  [x]  help - List available commands

-  [x]  Database storage of speaker and assignment information

-  [ ] Speaker response handling (accept/deny)
-  [ ]   Administrator notification of speaker responses


               
# Installation
**1. Clone the repository:**
  
```bash 
  git clone https://github.com/yourusername/sms_service.git

  cd sms_service
```
**2. Install Diesel CLI if you haven't already:**

```bash

cargo install diesel_cli 

```
**3. Build the Project:**
``` bash
  cargo build --release
```

**4. Set up configuration:**

Create a .env file in the project root with your Twilio credentials and other settings:

```dotenv
# Twilio Configuration
TWILIO_ACCOUNT_SID=your_account_sid_here
TWILIO_AUTH_TOKEN=your_auth_token_here
TWILIO_PHONE_NUMBER=your_twilio_phone_number

# Database Configuration
DATABASE_URL=postgres://username:password@localhost/database_name
```

**5. Set up and migrate the database:**
```bash
# Create the database (if not already created)
createdb sms_service

# Generate a new migration (if needed)
diesel migration generate speakers

# Run migrations
diesel migration run

```
Usage
```bash
# Adding a speaker
./target/release/sms_service add-speaker --name "John Doe" --phone "verified no that you have set in twilio" --date "2025-03-01T14:00:00Z"

# Get help
./target/release/sms_service --help
```