# The Mission

> “I come with empty hands and the desire to unbuild walls.”

― Ursula K. Le Guin, The Dispossessed

The Tehanu text editor is an opinionated fork of the Zed text editor. This
document tries to explain why this fork exists.

## Performance, but at what cost?

Coding is more than just productivity. We need tools that are fit for purpose,
that aren't beholden to investors or share holders. There are aspects of the Zed
code editor that I think are wrong in the moral and ethical sense, and there are
aspects that I think are simply bad choices from a technical perspective.

I don't want AI features in my editor. It's not just because I think they are
bad for productivity, performance and quality (I do), but because I think the
companies behind the technology and the _cost_ of the technology make it not
only unsustainable but unethical.

I also object to making myself and my work dependent on paying a subscription
fee. I don't want an outage at Anthropic to affect my ability to do my work. I
think it is a grave mistake to build anything on such shaky foundations as the
sustainability and profit margins of the AI industry.

But ignore all that. I think these are bad features in a code editor, period.

I don't want multiplayer editing integrated into the editor. Pair programming
works just fine without having two people editing the same file at the same
time. Assigning one person to think is a good thing.

I don't want my software to update itself automatically, or to download binaries
without me explicitly asking it to.

I don't want to sign in, or sign up, or join the community. I want my tools to
be hammers, screwdrivers and saws. I don't need my hammer to come with a
subscription fee.

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

- [Flow Editor](https://flow-control.dev) - A TUI-based and very promising
  editor with planned collaboration features.
- [Neovim](https://neovim.io)
- [Emacs](https://www.gnu.org/software/emacs/)
- [Kate](https://kate-editor.org/)
