# Project description

This is a trivial program that turns any english string to pig latin. Pig latin, as defined in the Rust book, is the following:

    The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

# How to run it

Clone the repo and make sure you have Rust installed in your VSCODE. Just type in 'cargo run' and you'll see it on your terminal. If you use a different I.D.E., provided it can run Rust, then the process is likely similar.

# Problems (that I know of)

1. It currently doesn't handle symbols that may be at the end of a word like commas and periods.
2. There may be some rare instances where a word can start with a non-letter. As for such instances, I don't know if I should even take them into account because a) I can't think of any examples, and b) they may be so rare that it's not worth handling them.
3. It may be worth it in the future to not turn every letter to lowercase and instead return the pig latin conversion string in its original letter cases.

# Future fixes/possible changes

1. Fixing everything in the problems section.
2. As usual, the possibility of expanding this program and creating interactable UI like text fields is there.

# Notes on pig latin and what counts as a vowel

1. Pig latin's definition, as stated in the Rust book, does not seem to be universal for there are multiple examples of pig latin translators on the internet that don't append -hay at the end of words that start with vowels.
2. I did some cursory research on whether 'y' or other consonants could or should count as vowels. As far as I could tell, a word that starts with 'y' like 'your' doesn't have 'y' as a vowel. However, the case with 'w' seemed to be more perplexing. For example, the 'w' in 'wonderful' is counted as a vowel because of some linguistic rules in english. Implementing such cases in my program seemed to be extremely annoying (or relatively annoying, if you don't want me to be hyperbolic), so I chose to ignore them. If you've read this far, then please forgive me for not implementing such cases. I may still do so in the future if I feel like it.
