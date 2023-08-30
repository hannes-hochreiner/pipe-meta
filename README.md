# PIPE-META
A tool to generate meta-data from the data being piped through.
In the current version, the file length and the sha256 hash are provided.

## Usage
The program accepts one argument, which is the file to which the meta-information is written.

```bash
cat test_file.txt | pipe-meta --meta-file output.json | some-other-program
```

The output is a JSON file containing the file length (as usize) and the sha256 hash (as hex string).

```json
{
  "length":534,
  "sha256":"1a122cbd4fc9c4efc3c0eb76f2f54745492dc673a914013a43dc816151a00f9f"
}
```

## License

This work is licensed under the MIT or Apache 2.0 license.

`SPDX-License-Identifier: MIT OR Apache-2.0`