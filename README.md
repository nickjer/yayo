# Yayo

[![Latest Version](https://img.shields.io/crates/v/yayo.svg)](https://crates.io/crates/yayo)
[![Downloads](https://img.shields.io/github/downloads/nickjer/yayo/total.svg)](https://github.com/nickjer/yayo/releases)
[![License](https://img.shields.io/github/license/nickjer/yayo.svg)](https://github.com/nickjer/yayo)
[![Continuous Integration Status](https://github.com/nickjer/yayo/workflows/Continuous%20integration/badge.svg)](https://github.com/nickjer/yayo/actions)

A command line interface (CLI) used to manage and generate OTP (one time
password) codes for your various accounts.

## Installation

Install using [cargo]:

```shell
cargo install yayo
```

## Pre-compiled Binaries

An alternative method is to download and run the pre-compiled binaries:

<https://github.com/nickjer/yayo/releases>

## Usage

Check the help (`--help`) for details on using this tool:

```shell
Manage and generate OTP (one time password) codes

Usage: yayo <COMMAND>

Commands:
  add         Add a new account
  completion  Output shell completion
  delete      Delete an account
  list        List all accounts
  view        View the code for an account
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Add Account

The `add` subcommand is used to add an account:

```shell
Add a new account

Usage: yayo add [OPTIONS] <ACCOUNT> <SECRET>

Arguments:
  <ACCOUNT>  Name of the account
  <SECRET>   Secret key used to generate code

Options:
  -a, --algorithm <ALGORITHM>  Algorithm used to generate code [env: YAYO_ALGORITHM=] [default: SHA1] [possible values: SHA1, SHA256, SHA512]
  -d, --digits <DIGITS>        Number of digits composing code [env: YAYO_DIGITS=] [default: 6]
  -s, --step <STEP>            Duration in seconds of step [env: YAYO_STEP=] [default: 30]
  -h, --help                   Print help
```

> *Note:* For added security it stores all secrets in your operating system's
> local keyring.

**Example:** Add your GitHub account with provided secret (`XXXXXXX`):

```shell
yayo add github XXXXXXX
```

Although it is not recommended, you can alter any of the arguments used in the
OTP code generation (`algorithm`, `digits`, and `step`) as command line options
or through environment variables.

### Delete Account

The `delete` subcommand is used to delete a previously added account:

```shell
Delete an account

Usage: yayo delete <ACCOUNT>

Arguments:
  <ACCOUNT>  Name of the account

Options:
  -h, --help  Print help
```

**Example:** Delete the previously added GitHub account:

```shell
yayo delete github
```

### List All Accounts

The `list` subcommand is used to list all available accounts and their provided
configurations:

```shell
List all accounts

Usage: yayo list

Options:
  -h, --help  Print help
```

### View Account

The `view` subcommand is used to output the OTP code generated for the
requested account:

```shell
View the code for an account

Usage: yayo view <ACCOUNT>

Arguments:
  <ACCOUNT>  Name of the account

Options:
  -h, --help  Print help
```

**Example:** View the OTP for the GitHub account:

```console
$ yayo view github
389200
```

### Command Completion

The `completion` subcommand is used to output a shell script used to support
command completion for this binary:

```shell
Output shell completion

Usage: yayo completion <SHELL>

Arguments:
  <SHELL>  Name of the shell [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help  Print help
```

**Example:** Output command completion for the fish shell:

```shell
yayo completion fish | source
```

Now you can use tab-completion with `yayo`.

## Linux (Gnome) - Keyboard Shortcut to Clipboard

This is a simple walkthough on setting up the F12 key to populate the clipboard
with the OTP code. This makes logging into GitHub or any other service as
simple as pushing F12 and pasting the contents of the clipboard into the 2FA
input.

1. You will need `xsel` or some other CLI that can read the contents of STDOUT
   to the clipboard:

   ```shell
   sudo apt install -y xsel
   ```

2. If using Gnome 3 you can just open the control center and navigate to the
   "Keyboard Shortcuts" with:

   ```shell
   gnome-control-center
   ```

3. Scroll to the bottom and set a "Custom" keyboard shortcut with the following
   options:

   > Name: **OTP (GitHub)**
   > Command: `sh -c '~/bin/yayo view github | xsel --primary'`
   > Shortcut: **F12**

[cargo]: https://doc.rust-lang.org/cargo/
