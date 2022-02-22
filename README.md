# Repository

**Repository** (or Repo for short) is an open, extensible, and fast **personal object store** with rich metadata support and a versatile query language.

**Current status**: Work In Progress

**License**: GPLv2+

## Design Goals

- Easy to use as a command-line program, making it easy to integrate Repo into other applications.
- Extensible plugin system to allow pre- and post-processing of data.
- Two-way-sync among multiple devices with E2E encryption.
- Stable ID, but mutable contents.

## Concepts

In Repo, each object is described by nothing but a unique ID. There are no directories or paths.

In its simplest form, a Repo store is a list of such IDs, and each ID is linked to a real file stored somewhere on your disk.

This simplest system is not user-friendly. You absolutely want to summarize a file's contents in its file name. Repo implements a much more general concept, called _attribute_. With Repo, you can associate an object with any number of attributes. For example, you can

1. get back file paths by adding an attribute called `path`
2. adding information (artist, album, ...) to an MP3 file

On top of that, Repo allows you do fast queries on your database. For example, you can list all of your MP3 files with a one-line command. Of course, a query can be as complex as you'd imagine.
