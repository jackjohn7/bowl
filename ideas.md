# Ideas

This document tracks implementation ideas that I've come up
with for solving potential problems getting us closer to the
proposed application in the README.md.

## Bowl directory

The bowl directory is included in the bowl file, but not 
cloned when the user uses `bowl init`.

## Bowl.lock

Perhaps a bowl.lock file should be created optionally when
you use a template. If you don't it'd be difficult to tell
what version of the template you're using and make it nearly
impossible to match your version with that version's set of
commands after that template has been updated.

## Commands

I don't generally like the idea of running arbitrary code on 
someone's computer, but it's user initiated, and I don't see
how it could be done by accident. Templates should be 
moderated to help prevent malicious use of this feature, but
in general, the onus is on the user here. The javascript
world has a similar situation with npx as far as I can tell.

After using a command, the command is cached on your system
to be used later to minimize server load and preserve
consistent behavior. You can clear the cache to ensure you 
have the latest command if you want. 

### bowl.toml to add commands in the following format:

```toml
[command.add]
# This example uses arbitrary npm commands to facilitate this
# Realistically, you could do anything with this.
args = [
    # Declarative matching on arguments provided
    {
        value = "tailwind",
        branches = [{ # If a user is missing a required tool in one branch, another can be tried
            has = ["npm"],
            # some other conditions are "on_windows", "on_mac", "on_linux", "on_other"
            exec = [
                { cmd = "npx tailwindcss init" },
                { cmd = "<Maybe some file manipulation to add tailwind route>" },
                { cmd = "<Some other file manipulation to link tailwind css file to head>" },
            ]
        }]
    },
    {
        value = "postgres",
        branches = [ {
            has = ["go"],
            exec = [
                { cmd = "go get github.com/jmoiron/sqlx" },
                { cmd = "<Maybe some file manipulation to add db files>" },
                { cmd = "go install github.com/pressly/goose/v3/cmd/goose@latest" },
                { cmd = "<add local configurations to .env file>" },
                { cmd = "goose create add_schema sql" },
            ]
        }]
    }
]
```

I would also like to allow users to include scripts in their
templates that can be referenced in these commands. You could
have something similar to this:

```toml
[command.add]
# any arguments following the `add` string are passed as arguments to
#  the program/script.
# This way, you can implement the logic in your preferred programming
 # language rather than this TOML DSL.
# Note, any files included in a command with the `script` property 
#  will be included in the produced bowl file when publishing the
#  template. They will not be included in your project when it is
#  cloned down with the `init` command though.
branches = [
    { has = "go", script = "bowl/scripts/add.go", exec = "go run {{script}} {{...args}}" },
    { has = "rustc", script = "bowl/scripts/add.rs",  build = "rustc -C opt-level=3 {{script}}" },
    { has = "node", script = "bowl/scripts/add.js" },
    { has = "deno", script = "bowl/scripts/add.js" },
    { has = "bun", script = "bowl/scripts/add.js" },
    { has = "python3", script = "bowl/scripts/add.py" }
    # These are tried in order. If a user has none of these required
    # technologies, the user is informed that they should install
    # one of these if they which to use the command.
]
# Of course, I would recommend using a scripting language relevant to
# the project template. A JS project likely implies that the user has
# some sort of js runtime.
```

These external scripts/programs will make it much easier
