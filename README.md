# seedsplit
A simple and information-theoretic secure seed phrase splitting scheme.

... And mostly just an exercise for me to practice Rust. Because you probably shouldn't be putting your seedprhase on a CLI you found on github.

## Algorithm
The algorith is explained in [this](https://bitcoin.stackexchange.com/a/65434) stackexchange answer.

## Zero indexed
There might be some confusion if the word list is zero or one indexed. As stated in [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#generating-the-mnemonic):

> bits of its SHA256 hash. This checksum is appended to the end of the initial entropy. Next, these concatenated bits are split into groups of 11 bits, each encoding a ***number from 0-2047, serving as an index into a wordlist***. Finally, we convert these numbers into words and use the joined words as a mnemonic sentence.

## Usage
First, download the repo:
```
$ git clone https://github.com/derch28/seedsplit.git
```
The CLI has two functionalities: Split a seedphrase and Rebuild a seedphrase (from the keys you get as a result of splittin it).

### Split
```
$ cargo run -- split [seedphrase]
```
This gives you 3 key parirs. A1-B1, A2-B2, A3-B3. To get your seedphrase back use rebuild:

### Rebuild
```
$ cargo run -- rebuild [key_1] -- [key_2]
```

## Example
Split
```
~/seedsplit$ cargo run -- split custom belt rotate wish stove buffalo powder potato material scrap horn cushion
   Compiling seedsplit v0.1.0 (/home/derch/seedsplit)
    Finished dev [unoptimized + debuginfo] target(s) in 3.23s
     Running `target/debug/seedsplit split custom belt rotate wish stove buffalo powder potato material scrap horn cushion`

---------------------------------------------------------------------
A1: vicious mother whip recycle camp winter strong thing middle artwork patch cost
B1: drink luggage scrub emerge remove canvas spirit sight witness reform smoke alarm
---------------------------------------------------------------------
A2: actor gate region guilt grape peanut armor salt alley ancient tent melt
B2: crop regret amateur neutral ill knock outer tuition list rib mixture profit
---------------------------------------------------------------------
A3: toss seven nuclear elbow iron process similar regular obscure child where globe
B3: extend expect ceiling rely gather impulse theory village vague office infant spring
---------------------------------------------------------------------
```
Rebuild using key pair 1:
```
~/seedsplit$ cargo run -- rebuild vicious mother whip recycle camp winter strong thing middle artwork patch cost -- drink luggage scrub emerge remove canvas spirit sight witness reform smoke alarm
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/seedsplit rebuild vicious mother whip recycle camp winter strong thing middle artwork patch cost -- drink luggage scrub emerge remove canvas spirit sight witness reform smoke alarm`

---------------------------------------------------------------------
Seedprhase:custom belt rotate wish stove buffalo powder potato material scrap horn cushion
---------------------------------------------------------------------
```
Rebuild using key pair 2:
```
~/seedsplit$ cargo run -- rebuild actor gate region guilt grape peanut armor salt alley ancient tent melt -- crop regret amateur neutral ill knock outer tuition list rib mixture profit
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/seedsplit rebuild actor gate region guilt grape peanut armor salt alley ancient tent melt -- crop regret amateur neutral ill knock outer tuition list rib mixture profit`

---------------------------------------------------------------------
Seedprhase:custom belt rotate wish stove buffalo powder potato material scrap horn cushion
---------------------------------------------------------------------
```
Rebuild using key pair 3:
```
~/seedsplit$ cargo run -- rebuild toss seven nuclear elbow iron process similar regular obscure child where globe -- extend expect ceiling rely gather impulse theory village vague office infant spring
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/seedsplit rebuild toss seven nuclear elbow iron process similar regular obscure child where globe -- extend expect ceiling rely gather impulse theory village vague office infant spring`

---------------------------------------------------------------------
Seedprhase:custom belt rotate wish stove buffalo powder potato material scrap horn cushion
---------------------------------------------------------------------
```
