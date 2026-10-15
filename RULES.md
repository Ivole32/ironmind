# Rules

## 1. AI usage

**1.1** Using AI in any way is not allowed, except for the following, and only in documentation and other written text (never in code):
- correcting spelling or grammar
- phrasing things more clearly without changing their meaning
- polishing the README and other documents that explain the project (e.g. layout, design, and structure)

**1.2** As an exception to 1.1, I may use AI and other tools to extract training and testing data from public datasets and convert it into a format I can use. Unfamiliar file formats are outside the scope of this challenge. Any code or tools used for this must stay separate from the neural network code and must be documented.

## 2. Research

**2.1** I may only use the official Rust documentation at [doc.rust-lang.org](https://doc.rust-lang.org). The only exception is documentation for external Rust crates I use, which may be hosted elsewhere.

**2.2** I may only research math topics that are clearly unrelated to neural networks or anything closely related to them.

**2.3** I may use public datasets (e.g. MNIST) for training and testing.

## 3. Documentation

**3.1** I have to document as much of my thought process, research, and development process as possible.

**3.2** After the initial commit containing this rulebook, every change to it must be documented.

**3.3** I have to document every dataset I use, including a correct citation and attribution as required by its license.

**3.4** I have to document all math research I do, including what I looked up, why I needed it, and which sources I used.

## 4. Dependencies

**4.1** I will use as few external Rust crates as possible, as far as my experience allows.

## 5. Completion

**5.1** I split my data into a training set and a testing set. The network is never trained on the testing set.

**5.2** Once I have chosen my training and testing sets, I may not change either of them because my code performs poorly on them. Any change for another reason must be documented.

**5.3** The challenge is done when the network recognizes at least **95%** of the testing set correctly.

**5.4** Even after reaching this goal, the challenge is only done when I can't find any further ways to optimize the project on my own.
