# Taskigt - hierarchical planning and note taking

A structured text format for simple hierarchical (tree based) note taking and planning. And a web interface to edit
and persist documents locally.

Philosophy/goals:
* Function over form
* Ideas/notes form hierarchies
* Support unstructured stream of consciousness note taking
* Be able to store as a text file (not losing any details)
* Be able to import any text file (preferrably list indented)

## Example

https://blofroth.github.io/taskigt/

## Disclaimer
Alpha software, may eat your laundry (notes). It shouldn't, but it might.

## FAQ

* Why is the example so ugly?
  * To emphasize function over form! (and because I'm lazy)
* Why did you make this? (Why aren't you using orgmode, ...)
  * Mostly as a small project to learn Rust better and try out WASM web development.
  * But also since I've been using a light-weight text-based note taking format like this for a while. While pure text
    files have their merits (e.g. simplicity) I wanted to see if I could take better advantage of the inherent ad-hoc
    structure

## Planned features/ideas
* Search
* Reports per item category
* Diary notes support
  * Migration of unfinished tasks from previous days
* Possibly a CLI tool to interact with text files
* Online synchronization

## Stack
* Rust
* Wasm
* Yew

## Inspiration
* Voidmap
* Tomboy
* Markdown (somewhat)


