# QOTD-rust
## What
[RFC 865](tools.ietf.org/html/rfc865) compliant Quote of the Day service written in Rust.  
Listens on port 17 for TCP and UDP connections and replies with a random quote picked from a provided file

## Usage
### Build
```
git clone https://github.com/mimetypes/qotd-rust
cd qotd-rust/
cargo build --release
```

### Run
Needs root since we're binding to a port below 1024
```
sudo ./qotd-rust <quotes_file>
```

### Quotes file
Quotes are delimited with a `%`, newlines and tabs are respected. e.g.
```
Generic quote
%
>So, just like fortune(6) files?

>Yes
%
Thank you for contacting customer service
    - Customer service
%
Insufficent entropy
```

## Limitations
- No logging and rather rudimentary error handling
- Doesn't enforce or check RFC 865 recommended syntax for quotes (ASCII only and 512 chars max)

## Security
Does currently not implement any measures against ping pong or other sorts of DoS attacks.  
Should be safe otherwise as it does not interact with incoming data in any meaningful way.

## Why
Mainly to get started with Rust, it's also one of the few RFCs I could read in its entirety during a lunch break.
