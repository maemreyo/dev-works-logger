# git-stats-bot
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
