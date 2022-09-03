<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<br />
<div align="center">
  <a href="https://github.com/95jonpet/cmdrec">
    <img src="doc/logo.svg" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">cmdrec</h3>

  <p align="center">
    Record and retrieve command results.
    <br />
    <a href="https://github.com/95jonpet/cmdrec"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/95jonpet/cmdrec">View Demo</a>
    ·
    <a href="https://github.com/95jonpet/cmdrec/issues">Report Bug</a>
    ·
    <a href="https://github.com/95jonpet/cmdrec/issues">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## About The Project

`cmdrec` is a command line tool for recording, and later reusing, command results.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [![Rust][rust-shield]][rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

`cmdrec` can be installed locally using [cargo](https://doc.rust-lang.org/stable/cargo).

1. Clone the repo.

    ```bash
    git clone https://github.com/95jonpet/cmdrec.git
    ```

2. Compile and run the code.

    ```bash
    cargo install --path .
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

The following example illustrates typical usage of `cmdrec`:

```bash
# Record test results.
record="$(cmdrec record -- cargo test)"

# Print errors if the `cargo test` command fails.
if [[ "$(cmdrec status "${record}")" -ne 0 ]]; then
  echo "ERROR: Tests failed!" >&2
  cmdrec stderr "${record}"
  exit 1
fi

# Append the test output to a file.
cmdrec stdout "${record}" >> full-output.log
```

For a complete list of options, refer to the built-in help:

```bash
cmdrec --help
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project.
2. Create your Feature Branch (`git checkout -b feature/amazing-feature`).
3. Commit your Changes (`git commit -m 'Add amazing feature'`).
4. Push to the Branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/95jonpet/cmdrec.svg?style=for-the-badge
[contributors-url]: https://github.com/95jonpet/cmdrec/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/95jonpet/cmdrec.svg?style=for-the-badge
[forks-url]: https://github.com/95jonpet/cmdrec/network/members
[stars-shield]: https://img.shields.io/github/stars/95jonpet/cmdrec.svg?style=for-the-badge
[stars-url]: https://github.com/95jonpet/cmdrec/stargazers
[issues-shield]: https://img.shields.io/github/issues/95jonpet/cmdrec.svg?style=for-the-badge
[issues-url]: https://github.com/95jonpet/cmdrec/issues
[license-shield]: https://img.shields.io/github/license/95jonpet/cmdrec.svg?style=for-the-badge
[license-url]: https://github.com/95jonpet/cmdrec/blob/main/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/95jonpet
[rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=white
[rust-url]: https://www.rust-lang.org
