# jira release stats

> a tool for mining information out of jira versions

## configuration

The following env vars are required

| Name           | Description                    |
|----------------|--------------------------------|
| JIRA_HOST      | jira host ( including scheme ) |
| JIRA_USERNAME  | jira username                  |
| JIRA_PASSWORD  | jira password                  |
| PROJECT        | jira project id                |


## Usage

For now, just run as a cargo main

```bash
$ JIRA_HOST=xxx JIRA_USERNAME=xxx JIRA_PASSWORD=xxx PROJECT=xxx cargo run
```
