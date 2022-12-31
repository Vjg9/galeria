<div align="center">
	<img src="assets/logo.svg" width=170 </img>
</div>
<h1 align="center">Galeria</h1>
<h5 align="center">A microservice for managing static files written in Rust</h5>

## Introduction
- __Galeria__ is written in the [Rust](https://rust-lang.org) programing language using the [Actix Web](https://actix.rs) framework.
- All user data is stored in a [Postgresql](https://postgresql.org) database.
- __Static files__ are stored in directories and served using __Actix Files__.
- All data is seperated in 3 tables: __Profiles__, __Albums__ and __Images__.

## The Api
- The __API__ consists of 4 basic routes: __profile__, __album__, __image__ and __static__

| /api/profile/           | Usage                                                     |
| ---                     | ---                                                       |
| get/name/{profile_name} | Return a profile from a name                              |
| list                    | Lists all existing profiles                               |
| add                     | Adds a profile from a name speciefied in the request body |
| delete/{profile_id}     | Deletes a profile containing the id                       |
| update/{profile_id}     | Updates values from the profile containing the id         |
| {profile_id}/album/list | Lists all profile's albums                                |

| /api/album/           | Usage                                                                     |
| ---                   | ---                                                                       |
| get/name/{album_name} | Return an album from a name                                               |
| add                   | Adds an album from a name and a profile id speciefied in the request body |
| delete/{album_id}     | Deletes an album containing the id                                        |
| update/{allbum_id}    | Updates values from the profile containing the id                         |
| {album_id}/image/list | Lists all album's images                                                  |

| /api/image/            | Usage                                                                        |
| ---                    | ---                                                                          |
| get/name/{image_name}  | Return an image from a name                                                  |
| add                    | Adds an image from a name and an album id speciefied in the request body     |
| delete/{image_id}      | Deletes an image containing the id                                           |
| update/{image_id}      | Updates values from the image containing the id                              |
| file/add               | Adds a static file from the multipart form data to a corresponding directory |
| file/delete/{image_id} | Deletes the image's static file from the id                                  |

- The __/static/__ route serves all directories from the static folder.
- Getting a static file from __/static/__: **api/static/{profile_name}/{album_name}/{image_name}.png**.

## Build

### Binary

- To build the project you will need __rust__.
- To clone the repository you will need __git__.

```zsh
git clone https://github.com/Vjg9/galeria 
cd galeria/server
cargo build --release
```
- The binary is going to be located in __target/release/__.

### Docker Image

- To build the docker image you will need [docker](https://docker.com).
- To clone the repository you will need __git__.

```zsh
git clone https://github.com/Vjg9/galeria 
cd galeria/server
docker build -t galeria .
```

## Run

### Docker Compose 

- To run the project you will need __docker__ and [docker compose](https://docs.docker.com/compose).
- To clone the repository you will need __git__.
- You will also need the __galeria__ docker image and the [__postgres__ docker image](https://hub.docker.com/_/postgres/).

``` zsh
git clone https://github.com/Vjg9/galeria 
cd galeria
docker compose up -d 
```
