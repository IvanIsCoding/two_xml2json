# two_xml2json

This contains the code for XML to JSON that originally shipped with [celq](https://github.com/IvanIsCoding/celq).

In future releases, I plan to add better documentation, testing against more XML datasets, fuzzing, and more. But for now, I just wanted to expose celq's logic as a reusable library.

## Acknowledgments

Special thanks to:
- [xml2json-rs](https://crates.io/crates/xml2json-rs)

That was the crate that originally provided celq's XML to JSON support. Unfortunately, it's abandoned and `quick-xml` had a lot of releases in between with bug fixes. Otherwise I'd still use it.

### Name

The crate is named `two_xml2json` as a joke referencing the Fast & Furious' second instance. crates.io names cannot start with numbers, otherwise this would be `2xml2json`. I felt any other name would feel like namesquatting, so I picked a more amusing name.

## Large Language Models Disclosure

Many commits in this repository were co-authored by LLMs. All commits were guided and reviewed by a human. I tried my best to keep things simple and auditable.

All the documentation in the manual has been hand-crafted.

## License

This project is dual-licensed under the MIT License and Apache 2.0 licenses. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.

## Contributing

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in `two_xml2json` by you shall be dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache 2.0 license](LICENSE-APACHE). Any additional terms or conditions shall not apply.
