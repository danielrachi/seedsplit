# seedsplit
A simple and information-theoretic secure seed phrase splitting scheme.

## Algorithm
As explained in [this](https://bitcoin.stackexchange.com/a/65434) stackexchange answer:

One Time Pad or XOR is an elegant and [information-theoretic](https://en.wikipedia.org/wiki/Information-theoretic_security) secure way to split a BIP39 seed. It's a method simple to describe (apt for a will), easy to verify (trust only yourself) and the resulting shares are mnemonics thus easy to record.

Best of all it can be computed entirely with paper and pencil eliminating risks from malware. The method does not scale efficiently for n of m when m is large, but works well for n of n, 2 of 3 and possibly 3 of 5.

Consider an example of a three word mnemonic from the 2048 word [BIP-0039 dictionary](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt):

S = "night love grit"

We will split the seed S into two parts, A and B, such that A + B = S (where + is element wise [addition mod](https://en.wikipedia.org/wiki/Modular_arithmetic) 2048). First generate a random key A of the same length, say A = "steel siren layer". To find the second key B, go word by word subtracting the dictionary indexes mod 2048 of A from S:

1st: (night - steel) mod 2048 = (1197 - 1706) mod 2048 = 1539 = scare

2nd: (love - siren) mod 2048 = (1060 - 1612) mod 2048 = 1496 = road

3rd: (grit - layer) mod 2048 = (822 - 1011) mod 2048 = 1859 = tribe

Thus B = S - A = "scare road tribe". To retrieve S add the two keys together:

1st: (steel + scare) mod 2048 = (1539 + 1706) mod 2048 = 1197 = night

2nd: (siren + road) mod 2048 = (1496 + 1612) mod 2048 = 1060 = love

3rd: (layer + tribe) mod 2048 = (1859 + 1011) mod 2048 = 822 = grit

As promised, S = A + B. Even with infinite computing power A and B reveal zero information about S. Individually they are nothing but random numbers. 3 of 3 can be achieved by generating two random keys, say A and B. Then the third key C is found as:

C = S - A - B; giving S = A + B + C. This can be extended to n of n.

For 2 of 3 repeat the method three times. Each time use a different random key A; say A1, A2 and A3. This generates three keys B1, B2 and B3. So now we have:

A1 + B1 = S

A2 + B2 = S

A3 + B3 = S

Divide the keys like this:

Switzerland: A1, A2

Canada: A3, B1

New Zealand: B2, B3

Vires in Numeris!



