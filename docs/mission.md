# The Mission

> “I come with empty hands and the desire to unbuild walls.”

― Ursula K. Le Guin, The Dispossessed

![NO AI](https://rfmedia.b-cdn.net/no-ai-small.png)

The Gram text editor is an opinionated fork of the Zed text editor. This
document tries to explain why this fork exists.

## Performance, but at what cost?

Coding is more than just productivity. We need tools that are fit for purpose,
that aren't beholden to investors or share holders. There are aspects of the Zed
code editor that I think are wrong in the moral and ethical sense, and there are
aspects that I think are simply bad choices from a technical perspective.

At first, the promise of Zed seemed alluring. As someone coming from Emacs and
Vim (then Neovim), having access to a fast editor able to handle large Rust
projects without getting bogged down by the LSP whenever it kicks off which also
came with good support for Emacs- and Vim-style editing seemed exactly like the
thing I was looking for.

However, I never even managed to get the editor installed. When I tried to
install and got presented with the Zed End User License, I couldn't accept it. I
didn't want to agree to any of it. So I gave up.

Zed kept popping up. People I had respected left their jobs to go work for Zed,
and even worse - work on AI integration in Zed.

I don't want AI in my editor. I don't want AI in _anything_. It's not just
because I think AI is bad for productivity, performance and quality (I do), but
because I think the companies behind the technology and the _cost_ of the
technology make it not only unsustainable but unethical. Even if AI tools _did_
work to make their users more productive (which I doubt), I still don't want to
use them. At all.

I also object to making myself and my work dependent on paying a subscription
fee. I don't want an outage at Anthropic to affect my ability to do my work. I
think it is a grave mistake to build anything on such shaky foundations as the
sustainability and profit margins of the AI industry.

I think these are bad features to put in a code editor.

I don't want multiplayer editing integrated into the editor. Pair programming
works just fine without having two people editing the same file at the same
time. Assigning one person to think is a good thing.

I don't want my software to update itself automatically, or to download binaries
without me explicitly asking it to.

I don't want to sign in, or sign up, or join the community. I want my tools to
be hammers, screwdrivers and saws. I don't need my hammer to come with a
subscription fee.

At first, I tried to build some other efforts I found online to make Zed work
without the AI features just so I could check it out, but didn't manage to get
them to work. At some point, the curiosity turned into spite. I became
determined to not only get the editor to run without all of the misfeatures, but
to make it a full-blown fork of the project. Independent of corporate control,
in the spirit of Vim and the late Bram Moolenaar who could have added
subscription fees and abusive license agreements had he so wanted, but instead
he gave his work as a gift to the world and asked only for donations to a [good
cause close to his heart](https://en.wikipedia.org/wiki/ICCF_Holland) in return.

This is the result. Feel free to build it and see if it works for you. There
is no license agreement or subscription beyond the open source license of the
code (GPLv3). It is yours now.

## Features

- Open: No telemetry, AI or privacy-invading third party integrations.
- Performance: Focuses on helping you with your programming tasks instead of
  spending CPU and GPU cycles on communicating with AI services, sending your
  keystrokes to a company server in the US or convincing you to sign up for a
  subscription.
- Language-aware: Aims to support many major and many minor programming languages
  and tools out of the box.

## Other options

If you don't want to use an editor that has gone down the AI route but do want a
capable, general-purpose, open source text editor, what other options are there?

Here are some other editors that I have used and would consider reasonable
alternatives.

- [Flow Control](https://flow-control.dev) - A TUI-based and very promising
  editor with planned collaboration features.
- [Neovim](https://neovim.io)
- [Emacs](https://www.gnu.org/software/emacs/)
