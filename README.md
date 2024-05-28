[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=voiceapiai_alertsinua-cli&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=voiceapiai_alertsinua-cli) [![Coverage Status](https://coveralls.io/repos/github/voiceapiai/ralertsinua/badge.svg)](https://coveralls.io/github/voiceapiai/ralertsinua) ![Crates.io Version](https://img.shields.io/crates/v/ralertsinua-http) ![Crates.io License](https://img.shields.io/crates/l/ralertsinua-http) ![docs.rs](https://img.shields.io/docsrs/ralertsinua-http) [![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner-direct-single.svg)](https://stand-with-ukraine.pp.ua)

# ralertsinua

<p>Rust async API wrapper (<em>reqwest</em>) & <abbr title="Terminal User Interface">TUI</abbr> (<em>ratatui</em>) for <u>alerts.in.ua</u>

![screencast](https://raw.githubusercontent.com/voiceapiai/ralertsinua/main/docs/assets/screencast.gif)


## Introduction
The Alerts.in.ua API Client is a Rust library that simplifies access to the alerts.in.ua API service. It provides real-time information about air raid alerts and other potential threats.


## Installation

### Install prebuilt binaries via shell script
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/voiceapiai/ralertsinua/releases/download/v0.5.0/ralertsinua-installer.sh | sh
```
### Download prebuilt binaries from the [releases page](https://github.com/voiceapiai/ralertsinua/releases/latest)

#### NOTE
Linux binaries compiled with musl and are light-weight, call straight into the kernel without other dynamic system library dependencies, can be shipped to most linux distributions without compatibility issues, and can be inserted as-is into lightweight docker images such as static distroless, scratch, or alpine.

## Usage

⚠️ Before you can use this library, you need to obtain an API token by submitting an [API request form](https://alerts.in.ua/api-request).

⚠️ Provide token via environment variable `ALERTSINUA_TOKEN` or via `--token` flag. If empty, the library will try to ask you interactively one time.

```bash
export ALERTSINUA_TOKEN="your_token"; ralertsinua

# or

ralertsinua --token "your_token"

```

Default polling interval is 30 seconds. You can change it via `ALERTSINUA_POLLING_INTERVAL_SEC` env or `--interval` flag.

```bash
export ALERTSINUA_POLLING_INTERVAL_SEC=60; ralertsinua

# or

ralertsinua --interval 60
```

## License
MIT 2024

## Inspitation & Credits & Thanks
- ratatui's [async template](https://github.com/ratatui-org/templates/blob/main/component/README.md#async-template)
- rspotify [rsotify](https://github.com/ramsayleung/rspotify)
- echomap [echomap](https://github.com/pjsier/echomap)
- alerts_in_ua Python client [alerts-in-ua](https://github.com/alerts-ua/alerts-in-ua-py)

## History

<a href="https://star-history.com/#voiceapiai/ralertsinua&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=voiceapiai/ralertsinua&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=voiceapiai/ralertsinua&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=voiceapiai/ralertsinua&type=Date" />
 </picture>
</a>

---

*[TUI]: Terminal User Interface
