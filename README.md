# 🌙 Luna

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)

## About <a name = "about"></a>

Luna is a static site generator written in rust that takes in markdown and generates a fully working static site!

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

To build the project, you will need a working rust enviroment.

### Installing

To get the dev enviroment running, first clone the repo

```
git clone https://github.com/blobcode/luna
```

and then cd into it

```
cd luna
```

End with an example of getting some data out of the system or using it for a little demo.

## Usage <a name = "usage"></a>

Using luna is simple. Just use `luna new <name>` to generate a new project. This creates a folder with a luna.ini config file, and a `templates` and `posts` folder. In the templates folder, there are 3 files: home.html, post.html and posts.html. These are templates for your posts. In the posts folder, there are markdown documents, which have a front matter defining data and then the post body.