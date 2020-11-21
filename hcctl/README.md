# hcctl

Simple CLI tool to keep a track of your https://healthchecks.io account

## Usage

### List all your current checks

```shell
HEALTHCHECKS_TOKEN=<API key> hcctl list

 ID         | Name                  | Last Ping
------------+-----------------------+-----------------------------------
 <redacted> | gitout_sync           | 0 hour(s) and 3 minute(s) ago
 <redacted> | dl_msfjarvis_dev_sync | 0 hour(s) and 4 minute(s) ago
 <redacted> | email_sync            | 0 hour(s) and 4 minute(s) ago
```

### List the 10 latest pings from a check

```shell
HEALTHCHECKS_TOKEN=<API key> hcctl pings <check_id>

 Number | Time       | Type    | Duration
--------+------------+---------+------------
 #22280 | 21/11 6:0  | success | 29.384 sec
 #22279 | 21/11 6:0  | start   |
 #22278 | 21/11 5:45 | success | 29.814 sec
 #22277 | 21/11 5:45 | start   |
 #22276 | 21/11 5:30 | success | 31.149 sec
 #22275 | 21/11 5:30 | start   |
 #22274 | 21/11 5:15 | success | 30.364 sec
 #22273 | 21/11 5:15 | start   |
 #22272 | 21/11 5:0  | success | 31.320 sec
 #22271 | 21/11 5:0  | start   |
```
