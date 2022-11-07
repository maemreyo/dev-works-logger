# dev-works-logger
[![CI](https://github.com/maemreyo/dev-works-logger/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/maemreyo/dev-works-logger/actions/workflows/ci.yml)
[![Rust](https://github.com/maemreyo/dev-works-logger/actions/workflows/rust.yml/badge.svg)](https://github.com/maemreyo/dev-works-logger/actions/workflows/rust.yml)
Build a bot to collect data from Git

## What are we trying to build?
- A Bot (not for *Discord*)
- It will run every week/month
- It will check all the repositories for the organization (with *Github GraphQL*) for the most amount of PR for a single contributor
- Create an image for that person and it will post it to Twitter and Discord
- Create a docker
## `.env` params you must have
- GITHUB_ORG_NAME
- GITHUB_TOKEN
- TWITTER_TOKEN
- DISCORD_HOOK

## Image to post
Should follow:
- A picture of the organization owning the repo
- A text with the name of the organization on the picture
- A Picture of the contributor
- A text about the contributor achievement (in the picture)
- A text about the contributor (in the tweet)
- As much as information as possible on the contributor
- Awesome design gets extra points
  
*Bonuses*
- Add more things like most amount of issues, most amount of reviews and so on.
---
## What did I do?
### Utils
- [x] Write docs
  - [ ] How to run?
  - [ ] How to contribute?
  - [ ] Write the Makefile?
    - [ ] Welcome new comers
    - [ ] Set Git hooks dir
    - [ ] Install cocogitto tools for standardizing commits
- [x] Write REQUIREMENTS
- [x] Change the project name 
  - [x] From `git-stats-bot` to `dev-works-logger`
- [x] Refactor folder structure
    +-- main.rs
    +-- modules
    +---- git
    +------ mod.rs
    +------ models
    +-------- mod.rs
    +-------- module1.rs
    +-------- module2.rs
    +---- discord
    +------ mod.rs
    +------ models
    +-------- mod.rs
    +-------- module1.rs
    +-------- module2.rs
    +-- utils
    +---- init
    +------ mod.rs
### Developments
- [x] Manage env variables with `dotenv`
- [x] Fetch data from GraphQL Github with gql_client
- [x] Research GraphQL 
  - [x] Get all recent commits of a repo
  - [x] Get all recent active repos
- [ ] Refactor: instead of using gql_client, I plan to change it to reqwest with json body
  - [ ] Create my own request client
- [x] Research about Git hooks
- [ ] Twitter API
- [ ] Discord
  - [x] Enable Github Webhook for Discord
  - [ ] Discord Bot
- [x] Research CI/CD on Github Action
  - [x] Add script to check Rust before merging PRs
  - [x] Add script to deploy Docker Image to Heroku
  - [x] Improve Deploy script
    - [x] Divide the whole script into pieces
    - [x] Remove some parts being not used
  - [x] ***Disable CI for now, I plan to do drive the project with another direction.***
- [x] Cronjob
  - [x] Cronjob
- [x] Docker
  - [x] Build Docker Image
  - [ ] Research `docker-compose`
  - [x] Improve Docker Image
    - [x] Create a minimal docker image
    - [x] Cron job
    - [x] ENV variables not found
      - [x] Find a way to get a github name, token on CI
      - [x] Pass env variables when building docker image on CI
- [x] Deploy to Heroku
- [x] Plan to deploy to another server (Heroku is not free from ***28/11***)
    - [x] Docker (Auto build: 5$ per month)
    - [x] GCS
    - [x] ***Localhost is the best choice for now, until I can find the alt.***
- [ ] Send Email SMTP
  - [ ] Investigate
- [ ] Implement CLI to trigger some features
  - [ ] Overall configurations
    - [ ] Github
      - [ ] PAC
      - [ ] Username
    - [ ] 
  - [ ] Twitter commands
    - [ ] Config Twitter credentials
    - [ ] Switch ON/OFF feature
  - [ ] Email commands
    - [ ] Config Gmail credentials
      - [ ] `username`, `password`
    - [ ] Switch ON/OFF feature
  - [ ] Discord commands
    - [ ] Config Discord credentials
    - [ ] Switch ON/OFF feature
- [ ] Add scripts to cut down the time to deal with some dev parts
  - [ ] Docker
    - [ ] Build image
    - [ ] Run image
    - [ ] Exec image
    - [ ] Push image to Docker Hub
  - [ ] Release
    - [ ] Changelog generator
  - [ ] Development
    - [ ] Run debug


## How to run this project?

### Install
- Run `Makefile` with `make -f Makefile`
