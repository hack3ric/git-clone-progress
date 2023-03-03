This is an example of parsing `git clone` output and turn it into a progress bar.

### Why?

This is intended to use Git CLI as a project dependency rather than introducing libgit2 or gitoxide. It is useful when you really need to reuse the most common packages (say, in a minimal Linux distribution).

~~*then why introduce rust after all*~~

### Should it work?

This is super hacky, but should work as long as Git (and its server implementation) doesn't change its output much. If making things as compatible as possible is a thing to you, **don't do this. You have been warned.**
