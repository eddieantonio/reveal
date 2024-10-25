# Reveal

Interactively reveal lines from a file or `stdin`, one line at a time.

![An animation of reveal on the command line](https://eddieantonio.ca/i/reveal-seq.gif)

# Usage

You can reveal lines from a file:

    $ reveal example.txt

![An animation of reveal being used to display a file](https://eddieantonio.ca/i/reveal-file.gif)

Or you can reveal lines from standard input:

    cmd | reveal

![An animation of reveal being used to reveal cowsay](https://eddieantonio.ca/i/reveal-cowsay.gif)

# Motivating example

Imagine you're holding a raffle at a meetup. You're going to give away
3 prizes. All of the attendees have been assigned a number, from 1 to 30.

```bash
seq 30 | ...
```

Now shuffle the members (requires GNU [`shuf`][shuf]):

```bash
seq 30 | shuf | ...
```

Select only the first 3:

```bash
seq 30 | shuf | head -3 | ...
```

And now add suspense by using `reveal`!

```bash
seq 30 | shuf | head -3 | reveal
```

![An animation of reveal being used to 3 random numbers](https://eddieantonio.ca/i/reveal-shuf.gif)

[shuf]: https://www.gnu.org/software/coreutils/manual/html_node/shuf-invocation.html

# Copying

Copyright © 2024 Eddie Antonio Santos.

See `LICENSE` for details.
