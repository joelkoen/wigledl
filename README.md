# wigledl

Download all the data you uploaded to WIGLE in CSV format.

## Usage

You will need to get your API token, which you can find on your [account page](https://wigle.net/account). Press `Show my token`, and copy the value next to `Encoded for use:`.

Then, provide this token when running the download script. wigledl will download all files to the current directory, and skip over files that have already been downloaded.

```sh
$ wigledl YOUR_ENCODED_TOKEN
Found 12 uploads on page 0
Downloading 20240713-01234
...
```
