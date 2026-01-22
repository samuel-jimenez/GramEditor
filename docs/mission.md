# The Mission

> “I come with empty hands and the desire to unbuild walls.”

― Ursula K. Le Guin, The Dispossessed

The Gram text editor is a fork of the Zed text editor. This
document is an attempt to explain why the fork exists.

## Performance, but at what cost?

Programming is more than just productivity. We need tools that are fit for purpose,
that aren't beholden to investors or share holders. There are aspects of the Zed
code editor that I think are wrong in the moral and ethical sense, and there are
aspects that I think are simply bad choices from a technical perspective.

Let me begin by making a technical argument for this fork.

At first, the promise of Zed seemed alluring. I moved from Emacs to Neovim
because I started coding in Rust, and using the Rust LSP in Emacs was a very
frustrating experience at the time. Whenever the LSP did any work the editor
would freeze, drop keystrokes or otherwise behave badly.

Neovim was better in this aspect, but as soon as I worked on any larger rust
project I would have the same kind of experience. Not good. At the same time,
more and more languages assume that the editor can use a language server and
don't even try to support editors without it.

When Zed appeared promising an actually fast and smooth editing experience able
to handle large Rust projects without getting bogged down, and also came with
good support for Emacs- and Vim-style editing, it seemed like exactly the thing
that I ought to try.

However, I never even managed to get the editor installed. When I tried to
download and install the binaries from the Zed website or Github page I would
get presented with the Zed End User License. To install, I would have to accept
the license.

I read the license.

> PLEASE READ THESE TERMS AND CONDITIONS CAREFULLY BEFORE USING THE SERVICE OR
> SOFTWARE OFFERED BY ZED INDUSTRIES, INC. ("ZED", OR "WE"). BY ACCESSING OR
> USING THE SOLUTION (AS DEFINED BELOW) IN ANY MANNER, YOU ("YOU" OR "CUSTOMER")
> AGREE TO BE BOUND BY THESE TERMS (THE "AGREEMENT") TO THE EXCLUSION OF ALL
> OTHER TERMS.

OK, so if I agree to this license, I am waiving my right to use any software by
Zed Industries under any other license. So does this mean that I could no longer
use any of the code from Zed under an open source license?

> You agree that You shall not: (a) exceed the scope of the licenses granted in
> Section 2.1; (b) make copies of the Editor; (c) distribute, sublicense,
> assign, delegate, rent, lease, sell, time-share or otherwise transfer the
> benefits of, use under, or rights to, the license granted in Section 2.1; (d)
> reverse engineer, decompile, disassemble or otherwise attempt to learn the
> source code, structure or algorithms underlying the Editor, except to the
> extent required to be permitted under applicable law; (e) modify, translate or
> create derivative works of the Editor; or (f) remove any copyright, trademark,
> patent or other proprietary notice that appears on the Editor or copies
> thereof.

I cannot try to learn the source code, structure or algorithms underlying the
Editor?

I couldn't accept it. I didn't want to agree to any of it. So I gave up.

Zed kept popping up. People I had respected left their jobs to go work for Zed,
and not only that - to work on AI integration in Zed.

I don't want AI in my editor. I don't want AI in _anything_. It's not just
because I think AI is bad for productivity, performance and quality (I do), but
because I think the companies behind the technology and the _cost_ of the
technology make it not only unsustainable but unethical. Even if AI tools _did_
work to make their users more productive (which I doubt), I still don't want to
use them. At all.

I also object to making myself and my work depend on paying a subscription fee
to anyone. I don't want an outage at Anthropic to affect my ability to do my
work. I think it is a grave mistake to build anything on such shaky foundations
as the sustainability and profit margins of the AI industry.

I think these are bad features to put in a code editor. Why should a code editor
have a license agreement that flies in the face of open source? Why should my
code editor integrate with technology that I find distasteful?

Thinking is a feature. Generative LLM technology is not.

Another selling point of Zed is "multiplayer editing" and built-in
collaboration.

I don't want it. In fact, I think these are bad features to put in a code
editor. In my team, we use different operating systems, different editors and
work from different time zones. We keep meetings to a minimum, and when we do
pair programming or mob programming, one person is typing while the others
think.

Multiple people editing the same files at once while video chatting at the same
time sounds like a nightmare.

I don't want to sign in, or sign up, or join the community. I want my tools to
be hammers, screwdrivers and saws. I don't need my hammer to come with a
subscription fee.

In Zed, the editor will automatically update itself, downloading binaries from
the Zed website vetted only by Zed. Its extensions are binary blobs hosted by
Zed, automatically updated and downloaded without user control. Those extensions
will, in turn, download and install anything they feel like.

As a band aid for this, newer versions of Zed have copied the workspace trust
mechanism from VS Code. I think this is, once again, a bad solution to a problem
that should not exist.

I don't want my software to update itself automatically, or to download binaries
without me explicitly asking it to.

The editor can offer to download and run something, but it should tell me
exactly what it wants to download and from where, and it should ask first. In
Zed, everything bad is opt out: AI, Telemetry, auto updates - auto-installation
isn't even offered as something to opt out of.

Zed even copies the VS Code misfeature of auto-running tasks - meaning arbitrary
code. You can't even look at source code using those editors without risking
arbitrary code execution.

## Curiosity and spite

At first, I tried to build some other efforts I found online to make Zed work
without the AI features just so I could check it out, but didn't manage to get
them to work. At some point, the curiosity turned into spite. I became
determined to not only get the editor to run without all of the misfeatures, but
to make it a full-blown fork of the project. Independent of corporate control,
in the spirit of Vim and the late Bram Moolenaar who could have added
subscription fees and abusive license agreements had he so wanted, but instead
gave his work as a gift to the world and asked only for donations to a [good
cause close to his heart](https://en.wikipedia.org/wiki/ICCF_Holland) in return.

This is the result. Feel free to build it and see if it works for you. There
is no license agreement or subscription beyond the open source license of the
code (GPLv3). It is yours now, to do with as you please.

To be honest, I am happy in Neovim. For most projects I still prefer it over
this editor. Don't expect this project to keep pace with Zed or to be a
Zed-killer, unless other people take it and keep building from here. This
project is mainly therapeutic. I had to get it out of my system.

## Features

- Open: No telemetry, AI or privacy-invading third party integrations.
- Performance: Focuses on helping you with your programming tasks instead of
  spending CPU and GPU cycles on communicating with AI services, sending your
  keystrokes to a company server in the US or convincing you to sign up for a
  subscription.
- Integrated: Gram includes the documentation directly in the editor. Long term,
  the goal is to include language support for most languages without requiring
  the use of extensions.

## Other options

If you don't want to use an editor that has gone down the AI route but do want a
capable, general-purpose, open source text editor, what other options are there?

Here are some other editors that I have used and would consider reasonable
alternatives to Zed.

- [Flow Control](https://flow-control.dev) - A TUI-based and very promising
  editor with planned collaboration features.
- [Neovim](https://neovim.io) / [Vim](https://www.vim.org)
- [Emacs](https://www.gnu.org/software/emacs/)
