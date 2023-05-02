## Discussion

### Transformation Magnitude

On a high level, the Collatz conjecture seems intuitively strange. Given that the magnitude of the augmenting transformation performed with an odd integer ($3x + 1$), is larger than the reducing transformation performed with an even integer ($2x$), isn't it inevitable that a sequence is propelled to infinity by the larger augmentation? It becomes immediately obvious with some basic experimentation that this is not the case, and in fact the opposite occurs, with reducing steps overwhelming any sequence to 1.

### Subsequences with Collatz Parity

One way to generalize the reason why this is the case for any arbitrarily larger number is to consider a simple two-step subsequence. For any number in a Collatz sequence, we can tell immediately if it should be reduced or augmented by looking only at the last digit, as the parity of the ultimate digit defines the parity of the number overall. We can further surmise what the next step will be, before even performing the requisite transformation, by combining information about the ultimate digit with the parity of the penultimate digit, as this tells us what the ultimate digit will become (giving us both the next step and the next next step). Consider the case of an ultimate digit of 2. A number with 2 on the end will obviously be reduced, but to what? If the last two digits are 42, the transformed ultimate digit will be 1; if the last two digits are 72, the transformed ultimate digit will be 6. These cases generalize to (odd)2 -> 1, and (even)2 -> 6.

With this in mind, consider any step in a Collatz subsequence where the current integer $n$ at $t=0$ is considered by it's ultimate digit and penultimate digit parity:

$$
u_{n} = n \mod 10 \\
$$


$$
pu_{n} = parity(\lfloor(n \mod 100) / 10 \rfloor) \\
$$

Mapping any integer represented as $(pu_{n})u_{n}$ to a table of initial and next steps (and excepting the base case $n = 1$):

| Action at $t=0 \Rightarrow$<br>Action at $t=1 \Downarrow$ | Reduce | Augment |
|-----------------------------------------------------------|--------|---------|
| Reduce  | (e)0, (o)2, (e)4, (o)6, (e)8      | (e)1, (o)1, (e)3, (o)3, (e)5<br>(o)5, (e)7, (o)7, (e)9, (o)9       |
| Augment | (o)0, (e)2, (o)4, (e)6, (o)8      |         |

The first thing to notice is that while we never consecutively augment, for one fourth of all $(pu_{n})u_{n}$ combinations we do reduce twice. Essentially, we can say that the uncertainty in the conjecture is introduced purely by even numbers (which, at t=1, may or may not augment, while odd numbers will always reduce).

### Generalizing Collatz Parity Behavior

Consider two examples extending this reasoning an additional step to include t=2.

**(o)(e)1:**
```console
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
521
Step 1: 521 - (Augment)
Step 2: 1564 - (Reduce)
Step 3: 782 - (Reduce)
...
521 reduced to 1 in 124 steps. Took 0 seconds.
```
For a positive integer of any size with a Collatz parity of (e)(e)1, the first three steps of its sequence consist of an augmentation followed by two reductions. Thus: 

$$(o)(e)1: -> ARR -> (e)2$$


**(e)(e)1:**

```console
Input a positive integer to show its Collatz sequence.
Input 'random' to use a random integer.
Input 'range' to test a range of integers.
Input 'exit' to exit.
861
Step 1: 861 - (Augment)
Step 2: 2584 - (Reduce)
Step 3: 1292 - (Reduce)
...
861 reduced to 1 in 104 steps. Took 0 seconds.

```
$$(e)(e)1: -> ARR -> (o)2$$

Generalizing these results:

$$(q)(e)1: -> ARR -> (\neg{q})2$$

Because $pu_{n}$ of any subsequence will depend on upstream digit parity, precise behavior cannot be determined from a Collatz Parity of an order that removes digit information. That being said, this framework provides a means of mapping possible transformation paths and enables the question: what kind of Collatz Parity would a counterexample to the conjecture have?
