# Project Setup
This file explains the setup and initialization of this project for future reference

1. Create `main.go`
2. Setup Github repo & sync with local enviornment
3. Mod Init: `go mod init github.com/<github-username>/<repo-name>`
4. Build: `go build`. (Build & Run: `go build && ./<binary-name>`)
5. Local Environment Variables
    - Create `.env` file for local environment variables
    - Add `.env` file to `.gitignore`
    - Get the env variable in your Go code `os.Getenv(<var-name>)`