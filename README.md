# Hedwig

CLI tool to send requests

Supports:
- HTTP and HTTPS
- All HTTP methods, *in theory*
- Custom headers
- Following redirects
- HTTP as default protocol

## Usage

Usage: hedwig [OPTIONS] --url <URL>

Options:
      --method <METHOD>  HTTP method
      --url <URL>        target URL
      --header <HEADER>  header in the format "key: value"
      --fix              guess some parts of the URL, for example the protocol
      --follow           follow redirects
  -h, --help             Print help
  -V, --version          Print version

## TODO
- [ ] Body and content-type
- [ ] Query
- [ ] Test different HTTP methods

