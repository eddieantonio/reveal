# Reveal

Reveal lines from `stdin` one at a time, interactively.

![An animation of reveal on the command line](https://eddieantonio.ca/i/reveal-seq.gif)

## Usage

You can reveal one file:

    $ reveal file.example

![An animation of reveal being used to display
a file](https://eddieantonio.ca/i/reveal-file.gif)

Or you can reveal content from standard input:
    
    cmd | reveal 

![An animation of reveal being used to reveal
cowsay](https://eddieantonio.ca/i/reveal-cowsay.gif)

# Examples

Using `seq` and `shuf`:

    seq 10 | shuf | reveal

# Copying

Copyright © 2024 Eddie Antonio Santos.

See `LICENSE` for details.
