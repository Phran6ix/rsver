
 DNS RESOLVER written in Rust. Inspired by Jon Cricket Coding [Challenges](https://codingchallenges.fyi/challenges/challenge-dns-resolver) 


## USAGE

```
git clone git@github.com:Phran6ix/rsver.git

cd rsver

cargo build

cargo run www.google.com
```

## EXAMPLE
The terminal command
```
 cargo run www.google.com
```
will out put 

```
Original URL: www.google.com.


THE DOMAIN IP = 216.58.223.228

```

And comparing with "nslookup" will out put 
```
Server:         192.168.1.1
Address:        192.168.1.1#53

Non-authoritative answer:
Name:   www.google.com
Address: 216.58.223.228
Name:   www.google.com
Address: 2c0f:fb50:4003:802::2004

```

WORKS

