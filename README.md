# git-stats-bot
[![CI](https://github.com/maemreyo/git-stats-bot/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/maemreyo/git-stats-bot/actions/workflows/ci.yml)
[![Rust](https://github.com/maemreyo/git-stats-bot/actions/workflows/rust.yml/badge.svg)](https://github.com/maemreyo/git-stats-bot/actions/workflows/rust.yml)
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
- [x] Write REQUIREMENTS
- [x] Manage env variables with `dotenv`
- [x] Learn how to fetch data from GraphQL Github with gql_client
- [x] Create a function to get all the recent commit in a repo
- [x] Research GraphQL to get the recent commit over all repos
- [ ] Refactor: instead of using gql_client, I plan to change it to reqwest with json body
  - [ ] Create my own request client
- [x] Refactor folder structure
  - [x] +-- main.rs
  - [x] +-- git
  - [x] +---- models
  - [x] +------ mod.rs
  - [x] +------ module1.rs
  - [x] +------ module2.rs
  - [x] +---- mod.rs
- [x] Research about Git hooks
- [ ] Twitter API
- [ ] Discord
  - [x] Enable Github Webhook for Discord
  - [ ] Discord Bot
- [ ] (Optional) Research CI/CD on Github Action
- [x] Cronjob or Github Action?
  - [x] Cronjob
  - [ ] Github Action?
- [ ] Docker? (No ideas)

## How to run this project?

### Install
- Run `Makefile` with `make -f Makefile`
