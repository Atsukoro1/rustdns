# Rustdns

## Table of contents
- [Rustdns](#rustdns)
  - [Table of contents](#table-of-contents)
  - [Expected features](#expected-features)
  - [Usage](#usage)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Configuration](#configuration)
  - [Contributing](#contributing)
  - [Documents / infomation sources](#documents--infomation-sources)
  - [License](#license)
 
## Expected features
- Blocking annoying third party `ad/tracking domains`
- Fast caching using Redis
- Fully configurable with `.conf` file
- Fast and memory safe

## Usage
Currently in development, this section will be filled out later

## Requirements
In order to run this compile and run the resolver correctly, there are some tools that need to be installed before you start compiling.

1. Rust
    - Go to [Rust official website](https://www.rust-lang.org/tools/install) and install rust with the script.
2. Redis
    - If you want to install Redis from package repository or executable follow the guide from [from here]()
    - This project already has `docker-compose.yml`, so it is more comfortable to run Redis with Docker, use [following guide](https://docs.docker.com/engine/install/) to install and run Docker.

## Installation
Currently in development, this section will be filled out later

## Configuration 
A `config.toml` file is used to configure this DNS server. Documentation with information about the configuration of this resolver will be released soon.

## Contributing
Your contributions are always welcome! Before you create any pull request, create an issue and discuss the problem you want to solve or an enchancement with the community. 

## Documents / infomation sources
The code itself is well-documented, so if you want to know something, data source link and the section of the document is probably at the top of the function or struct, but if you don't want to read it in code, I will also include it here.

1. DOMAIN NAMES - IMPLEMENTATION AND SPECIFICATION By: Network Working Group of the IETF, c1987 [cit. 2022-9-14]. Available at: https://www.ietf.org/rfc/rfc1035.txt
2. IANA (Internet Assiged Numbers Authority) - Root hint, zone and nameserver files Available at: https://www.iana.org/domains/root/files

## License
The MIT License (MIT) 2022 - [Jakub Dorničák](https://github.com/atsukoro1).
