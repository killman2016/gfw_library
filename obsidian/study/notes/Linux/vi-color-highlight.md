## Task: Turn on color syntax highlighting

Open a file, for example open existing file called file.c, enter:

`$ vi file.c`

Now press ESC key, type “: syntax on” i.e. type as follows:

`:syntax on`

## Task: Turn off color syntax highlighting

To turn it back off, press ESC key, type : syntax off

`:syntax off`

## How do I make the syntax highlighting permanent?

You can edit ~/.vimrc file and add append vim command syntax on to it. This ensures that vim will start with color syntax highlighting option:

`$ cd   $ vi .vimrc`

Append the following line:

`syntax on`