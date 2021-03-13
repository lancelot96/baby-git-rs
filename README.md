# Baby-Git-rs

In 2005, Linus Torvalds published the germinal version of [Git](https://github.com/git/git/tree/e83c5163316f89bfbde7d9ab23ca2e25604af290), which we called **Baby-Git**.
Baby-Git could be thought of as a first rudimentary version of Git.
Although much less sophisticated and convenient than its grown-up version, Baby-Git nevertheless encapsulates the core ideas behind modern-day Git.
And this repo is the rust version of Baby-Git.

## Components

1. Objects
    - Blob object
    - Tree object
    - Commit object
2. An object database
3. A current directory cache
4. A working directory

## Build

```bash
# clone thie repo
git clone https://github.com/lancelot96/baby-git-rs.git
cd baby-git-rs
# build the source code using `cargo`
cargo build --release
# add the binary file to `$PATH` environment
```

## Usage

There are 7 commands in Baby-Git, which basicly have the similar commands in Git.

| Baby-Git     |       | Git             |
| ------------ | ----- | --------------- |
| init-db      | ----> | git init        |
| update-cache | ----> | git add         |
| write-tree   | ----> | git write-tree  |
| commit-tree  | ----> | git commit-tree |
| read-tree    | ----> | git read-tree   |
| show-diff    | ----> | git diff        |
| cat-file     | ----> | git cat-file    |

### Preparation

```bash
$ cd examples
$ ls
hello.txt changelog
$ cat hello.txt
Hello world!
cat changelog
Initial commit.
```

### Initialize the repository

```bash
$ baby-git init-db
defaulting to private storage area
$ ls .dircache
index objects
```

### Add a file to the cache

```bash
baby-git update-cache hello.txt
```

### Write a tree object to the object database

```bash
$ baby-git write-tree
1ddb3f3884ad449ab3d5f6dd4ba150d0f6eed6b3
```

### Read a tree object

```bash
$ baby-git read-tree 1ddb3f3884ad449ab3d5f6dd4ba150d0f6eed6b3
100644 "hello.txt" (ae9a2591921ae106286b687f7fb0761b774bdb58)
```

### Read the contents of a blob object

```bash
$ baby-git cat-file ae9a2591921ae106286b687f7fb0761b774bdb58
"temp_git_file_WmGVzA": blob
$ cat temp_git_file_WmGVzA
Hello world!
```

### Commit a tree object to the repository

```bash
$ baby-git commit-tree 1ddb3f3884ad449ab3d5f6dd4ba150d0f6eed6b3 < changelog
Committing initial tree 1ddb3f3884ad449ab3d5f6dd4ba150d0f6eed6b3
2684fbe702307cff3e28ad17a9672bef466d9c80
```

### Get the contents of a commit object

```bash
$ baby-git cat-file 2684fbe702307cff3e28ad17a9672bef466d9c80
"temp_git_file_yKopmM": commmit
$ cat temp_git_file_yKopmM
tree 1ddb3f3884ad449ab3d5f6dd4ba150d0f6eed6b3
author yuyy,,, <yuyy@yuyy-pc> 2021-03-14 03:46:01.437475755 UTC
committer yuyy,,, <yuyy@yuyy-pc> 2021-03-14 03:46:01.437475755 UTC

Initial commit.
```

### Show the differences between blob objects and working files

```bash
$ baby-git show-diff
"hello.txt": ok
$ echo 'hello world!' > hello.txt
$ baby-git show-diff
"hello.txt": ae9a2591921ae106286b687f7fb0761b774bdb58
--- -   2021-03-14 11:51:03.513561514 +0800
+++ hello.txt   2021-03-14 11:50:56.916556320 +0800
@@ -1 +1 @@
-Hello world!
+hello world!
```

### Parent commit objects

When committing a tree object using the commit-tree command, the user has the option of specifying parent commit objects.
Logically, the parent commit object would be the commit object that resulted from the previous commit-tree command.

### Branches

Parent commit objects can be used to create a branch in a repository.
A branch can be created by specifying a parent commit object for a particular commit.

### Merges

In Baby-Git, a merge of two branches can be performed by specifying the two parent commit objects of a commit object.

### Database directory path

- `SHA1_FILE_DIRECTORY` environment to specify a custom directory that contains the object database

### Commit environment variables

- `COMMITTER_NAME` to store the committer's name
- `COMMITTER_EMAIL` to store the committer's email address

## Reference

- Baby Git Guidebook for Developers - Jacob Stopak
- [The first version of Git](https://github.com/git/git/tree/e83c5163316f89bfbde7d9ab23ca2e25604af290)
