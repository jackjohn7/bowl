# Bowl.rs

![hand-drawn graphic of a bowl](assets/bowl.png)

This is a **WIP**. Some things here are just my musings about
what the project *should* be rather than what it is at this
current time. I believe this is a cool idea, and I'd like to 
explore it. For current functionality, the
[Creating a Template](#creating-a-new-template) section will
tell you how to use Bowl.

Bowl is a CLI tool for quickly bootstrapping applications from
predefined boilerplate templates. While this tool is written in
Rust and is meant to simplify my life putting some Rust
applications together, it's language/framework-agnostic.

It's important to know what Bowl is **not**. Bowl is not a
package manager or environment manager. It cannot install 
software for you. It cannot manage your environment variables 
or configurations. For this, you should look elsewhere. I 
recommend the tools below:

- [direnv](https://direnv.net/)
- [nvm](https://github.com/nvm-sh/nvm)
- [brew](https://brew.sh/)

You can initialize boilerplate projects (script will prompt you
for necessary data).

```
bowl new axum-askama
```

*Planned*: You can also define commands for your template.

```
bowl run bugwright/axum-askama add tailwind
bowl run bugwright/axum-askama add goose
```

# Soup.rs (planned other repo)

Soup acts as a public repository of bowl templates, although you
can make some templates private. If you need templates to be
private, you can also host your own repository!

## Web Interface

- Homepage has some links to various pieces of documentation and
the templates page.
- Templates page shows a paginated listing of packages sorted by most
recently published.
- Bar at the top of screen with 
- Automatically generated documentation for subcommands and
configuration options (`bowl doc --open` to preview).

## API

- Have routes to get info about a template
- Have routes to return paginated template lists sorted by most
recently published

## User Ideas

- Require secret token to publish changes.
- Allow user to login through GitHub, GitLab, etc.

# Creating a Template

To get started creating a bowl template, you can use the following:

```
bowl new my_template
```

This will create a `bowl.toml` file that specifies configurations
and metadata for your template. Make sure to configure the metadata
so your template can be found easily! This is also where the version
number of the template can be found (default 1.0.0).

At the moment, only locally saving bowl templates is supported. To
do this, run the following:

```
bowl publish --local
```

To then use your locally saved template, you can run the following:

```
bowl use my_template --local
```
