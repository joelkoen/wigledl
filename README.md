# wigledl

Download all the data you uploaded to WIGLE in CSV format.

## Usage

Pre-compiled binaries for Linux [are available](https://github.com/joelkoen/wigledl/releases). Make sure to `chmod +x wigledl`.

You will need to get your API token, which you can find on your [account page](https://wigle.net/account). Press `Show my token`, and copy the value next to `Encoded for use`. You will need to provide this encoded token to wigledl.

Files will be downloaded to the current directory, and files that already exist will be skipped.

```sh
$ wigledl YOUR_ENCODED_TOKEN
Found 12 uploads on page 0
Downloading 20240713-01234
...
```
