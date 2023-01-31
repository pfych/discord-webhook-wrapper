# Discord Webhook Wrapper
Wraps a network request to a discord webhook endpoint

## Usage
```
Usage: discord-webhook-wrapper [OPTIONS]

Options:
  -c, --config <CONFIG>      Path to config toml
  -u, --user <USER>          Use definition in config file
  -n, --name <USERNAME>      The bot users username
  -p, --picture <PICTURE>    A URL to the bot users profile picture
  -m, --message <MESSAGE>    Message
  -e, --endpoint <ENDPOINT>  Discord webhook endpoint
  -h, --help                 Print help
```

## Config file
```toml
[cicd]
name = "CI/CD Notifications"
picture = "https://example.com/example.png"
endpoint = "https://discord.com/api/webhooks/example"

[git]
name = "Git Notifications"
picture = "https://example.com/example.png"
endpoint = "https://discord.com/api/webhooks/example"
```

```sh
$ dww -c ~/.config/webhook.toml -u "cicd" -m "Pipeline complete!"
$ dww -c ~/.config/webhook.toml -u "git" -m "New commit detected!"
```
