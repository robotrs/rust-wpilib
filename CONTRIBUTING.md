# Contribution Guidelines
Thank for your interest in this project! If you want to contribute, there are a few things you need to know.

## What to contribute
 - Bug reports: If you find a bug in this library, please report it to the GitHub issue tracker! Chances are, if you hit
   a bug, someone else might - and if we know about the bug, we'll be able to fix it.
 - Feature requests: If there is a feature you want added, feel free to make a feature request on the issue tracker. It
   will then be discussed, on the issue, and one of three things will happen:
    - The feature request is closed because it is unnecessary, belongs in a different repository, or is otherwise not
      wanted in this project.
    - The feature request is left open for discussion but not explicitly approved or dismissed.
    - The feature request is approved and assigned a <feature> tag.
   A good feature request is for a single feature and should be as atomic as possible. We will usually be less receptive
   to feature requests that involve large or breaking changes in the library. It should also fit within the project's
   goals - for example, don't ask for a feature in rust-wpilib that better belongs in the controls project.
 - Documentation: If you are willing to document code, everyone using this library will be forever in your debt.
   Seriously. You can never have too much documentation. We use `rustdoc` for documentation, so all you have to do is
   write doc comments - look [here](http://rustbyexample.com/meta/doc.html) for more information.
 - Bux fixes: we will always welcome bug fixes, and they will usually be merged as soon as possible!
 - Features: only implement features for which there are approved feature requests, or your feature will be closed.

## The contribution process
This project uses a workflow based on GitHub's forks. To begin contributing, fork this repository! Then, follow this
process:
 1. Claim the issue in question by commenting on it. Make sure no one else is already working on the issue!
 2. Create a branch (on your fork) for the feature or bug fix you're working on. The branch should be named with dashes
    and lowercase letters only, in accordance with the feature or bug the branch corresponds to. If you are fixing a
    bug, prefix the branch name with fix- (i.e. fix-encoder-overflow). If you're implementing a feature, use feature-
    (i.e. feature-spi-gyro). If you're adding documentation, use doc- (i.e. doc-sim-gyro).
 3. Implement the fix or feature on your branch.
 4. Create a pull request of your branch to the main repository.
 5. Travis CI will automatically build your changes and run your tests. If your projects fails the CI build, the errors
    must be fixed before the next step.
 6. Other contributors will comment and give feedback on your pull request.
 7. Address the feedback and commit and push the new changes.
 8. Repeat steps 4-6 until there are requested changes left on the PR and at least one core contributor has approved of
    the changes.
 9. Merge the PR into master.

A core contributor must still go through the code review process to get code merged into the repository and should not
push directly to master.

# Licensing
This project is licensed under the MIT License. By contributing, you agree for your code to be distributed under the
terms of that license.

# Core Contributors
 - Kyle Stachowicz ([KyleStach1678](https://github.com/kylestach1678))
 - Lee Mracek ([m3rcuriel](https://github.com/m3rcuriel))
 - Wesley Aptekar-Cassels ([wesleyac](https://github.com/wesleyac))
