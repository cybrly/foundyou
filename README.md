# FoundYou

`FoundYou` is a powerful command-line application designed for Open Source Intelligence (OSINT) and social engineering purposes. It allows users to search and display voter records by state and name, leveraging publicly available voter databases to gather valuable information.

This app is simply a CLI wrapper/implementation of Stephen P. Morse's web app: https://stevemorse.org/voterrecords/voterrecords.html.


## Installation

Add `foundyou` to your Cargo.toml:

```toml
[dependencies]
foundyou = "0.1.4"
```

Then run:

```sh
cargo build --release
```

Or install directly with:

```sh
cargo install foundyou
```

## Usage

```sh
foundyou --state <state_code> <first_name_starts_with> <last_name>
```

### Example

```sh
foundyou --state OH Chr Neu
```

### States Supported

The following state codes are supported:

- AR: Arkansas
- CO: Colorado
- DE: Delaware
- DC: District of Columbia
- FL: Florida
- ID: Idaho
- MI: Michigan
- MO: Missouri
- MS: Mississippi
- NJ: New Jersey
- NY: New York
- NC: North Carolina
- NV: Nevada
- OH: Ohio
- OK: Oklahoma
- PA: Pennsylvania
- UT: Utah
- VT: Vermont
- WA: Washington
- WY: Wyoming

## Purpose

`FoundYou` is an essential tool for OSINT practitioners and social engineers who need to gather detailed information about individuals from publicly accessible voter records. This tool can assist in various activities including background checks, research, and reconnaissance.

## Features

- **State-Specific Queries**: Customize your search by selecting from a variety of supported states.
- **Formatted Output**: Displays results in a neatly formatted table for easy reading and analysis.
- **Versatile Use Cases**: Ideal for OSINT investigations, social engineering, and other information-gathering activities.

## Help

For detailed help, run:

```sh
foundyou --help
```

## Example Output

```
+-----------------+--------------------+--------------+---------+------------+-------------+
| First Name      | Last Name          | Address      | City    | State      | Zip         |
+-----------------+--------------------+--------------+---------+------------+-------------+
| Chris           | Neuwirth           | 123 Main St  | City    | OH         | 12345       |
| ...             | ...                | ...          | ...     | ...        | ...         |
+-----------------+--------------------+--------------+---------+------------+-------------+
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License.