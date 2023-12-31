# texrs
```
      ___           ___           ___           ___           ___
     /\  \         /\  \         |\__\         /\  \         /\  \
     \:\  \       /::\  \        |:|  |       /::\  \       /::\  \
      \:\  \     /:/\:\  \       |:|  |      /:/\:\  \     /:/\ \  \
      /::\  \   /::\~\:\  \      |:|__|__   /::\~\:\  \   _\:\~\ \  \
     /:/\:\__\ /:/\:\ \:\__\ ____/::::\__\ /:/\:\ \:\__\ /\ \:\ \ \__\
    /:/  \/__/ \:\~\:\ \/__/ \::::/~~/~    \/_|::\/:/  / \:\ \:\ \/__/
   /:/  /       \:\ \:\__\    ~~|:|~~|        |:|::/  /   \:\ \:\__\
   \/__/         \:\ \/__/      |:|  |        |:|\/__/     \:\/:/  /
                  \:\__\        |:|  |        |:|  |        \::/  /
                   \/__/         \|__|         \|__|         \/__/
```
The easy way to generate LaTeX projects, and including great templates for projects in LaTeX. Also provides a naive build system implementation, with syntax inspired by
Cargo.

Personal project for now - this isn't actually useful to anyone but me.

## FEATURES

- [x] Generate a LaTeX project structure from the command line.
- [x] Feature a nice CLI menu that shows options.
- [x] Implement a 'paper' template.
- [x] Implement a 'letter' template.
- [ ] Implement a multi-chapter book template.
- [ ] Implement a nice thesis template.
- [x] Implement a math notes template.
- [x] Read and write project config to a TOML file.
- [x] Initialize a Git repository including the document.
- [ ] Create a remote repository to host it.
- [x] Multiplatform. It should work on all the major operating systems (Arch, Fedora, Debian, BSD, and maybe even that MacOS thing. Non-standard OSes not supported, Windows!)
- [x] Build the project with the local LaTeX installation.
- [x] Actually work well.

## USAGE

To get started, run 
```bash
texrs interactive [NAME]
```
which starts an interactive prompt allowing you to select your document type, &c. Use the `--help` option to see other techniques.
