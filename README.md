# From junior to senior

<img src="https://cdn.rawgit.com/KolesnichenkoDS/from-junior-to-senior/master/sicp.jpg"
     width="350" />

My study plan for going from junior to senior software developer (i. e. from level 1 to level 3 of the
[Programmer Competency Matrix](http://sijinjoseph.com/programmer-competency-matrix)). Inspired by
[Google Interview University](https://github.com/jwasham/google-interview-university).

## Usage

```bash
# install rust toolchain
curl https://sh.rustup.rs -sSf | sh

# clone this repository
git clone https://github.com/KolesnichenkoDS/from-junior-to-senior
cd from-junior-to-senior

# edit the list
vim resources/todo.json

# render to markdown
cargo run -- resources/ -f json -F markdown -o README.md
```

If you want to suggest a resource, you can just [open an issue](https://github.com/KolesnichenkoDS/from-junior-to-senior/issues).

## Stats

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Books__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |

## Table of contents

- [Calculus](#calculus)
- [Functional Analysis](#functional-analysis)
- [Geometry](#geometry)
- [Logic](#logic)
  - [Linear Logic](#linear-logic)
    - [Linear Lambda Calculus](#linear-lambda-calculus)
- [Algebra](#algebra)
  - [Linear Algebra](#linear-algebra)
- [Category Theory](#category-theory)
- [Combinatorics and Probability](#combinatorics-and-probability)
- [Discrete Mathematics](#discrete-mathematics)
- [Type Theory](#type-theory)
- [Lambda Calculus](#lambda-calculus)
  - [Linear Lambda Calculus](#linear-lambda-calculus)
- [Pi Calculus](#pi-calculus)
- [Data Structures](#data-structures)
- [Algorithms](#algorithms)
  - [Basic Algorithms](#basic-algorithms)
    - [Sorting](#sorting)
  - [Complexity](#complexity)
- [Programming Languages](#programming-languages)
  - [Java](#java)
  - [Kotlin](#kotlin)
  - [Scala](#scala)
  - [C Sharp](#c-sharp)
  - [Haskell](#haskell)
  - [Rust](#rust)
- [Programming Platforms](#programming-platforms)
  - [JVM](#jvm)
  - [.NET](#net)
- [Programming Paradigms](#programming-paradigms)
  - [Object-Oriented Programming](#object-oriented-programming)
  - [Functional Progamming](#functional-progamming)
  - [Logic Programming](#logic-programming)
  - [Actor Model](#actor-model)
- [Design Patterns](#design-patterns)
- [Domain-Driven Design](#domain-driven-design)
- [Computer Architecture](#computer-architecture)
- [Systems Programming](#systems-programming)
  - [Operating Systems](#operating-systems)
    - [Linux](#linux)
    - [Windows](#windows)
    - [macOS](#macos)
- [Compilers and Interpreters](#compilers-and-interpreters)
  - [Compilers](#compilers)
  - [Interpreters](#interpreters)
  - [Garbage Collection](#garbage-collection)
- [Networks](#networks)
- [Web Development](#web-development)
  - [Frontend](#frontend)
  - [Backend](#backend)
    - [Model-View-Controller](#model-view-controller)
    - [Microservices](#microservices)
  - [Web Applications Security](#web-applications-security)
- [Databases](#databases)
  - [SQL Databases](#sql-databases)
  - [NoSQL Databases](#nosql-databases)
- [Concurrency and Parallelism](#concurrency-and-parallelism)
  - [Multithreading](#multithreading)
  - [SIMD](#simd)
- [Security](#security)
  - [Cryptography](#cryptography)
  - [Web Applications Security](#web-applications-security)
- [Blockchain](#blockchain)
- [Testing](#testing)


## Calculus

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Books__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
- :pencil: __Articles__
  - [x] :us: [An Intuitive Introduction to Limits](https://betterexplained.com/articles/an-intuitive-introduction-to-limits) by [Better Explained](https://betterexplained.com/)
- :book: __Books__
  - [x] :ru: [Краткий курс математического анализа](http://nuclphys.sinp.msu.ru/mathan/) by Lev Kudryavtsev


## Functional Analysis

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Geometry

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Logic

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
- :pencil: __Articles__
  - [x] :us: [Sequent Calculus](https://ncatlab.org/nlab/show/sequent+calculus) by [nLab](https://ncatlab.org/)

### Linear Logic

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&amp;leftFill=%2300ff00) |
- :pencil: __Articles__
  - [x] :us: [Linear Logic](https://ncatlab.org/nlab/show/linear+logic) by [nLab](https://ncatlab.org/)

#### Linear Lambda Calculus

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Algebra

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Linear Algebra

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Category Theory

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Combinatorics and Probability

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Discrete Mathematics

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Type Theory

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Lambda Calculus

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Linear Lambda Calculus

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Pi Calculus

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Data Structures

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Algorithms

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Basic Algorithms

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

#### Sorting

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Complexity

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Programming Languages

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Java

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Kotlin

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Scala

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### C Sharp

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Haskell

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Rust

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Programming Platforms

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### JVM

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### .NET

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Programming Paradigms

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Object-Oriented Programming

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Functional Progamming

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Logic Programming

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Actor Model

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Design Patterns

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Domain-Driven Design

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Computer Architecture

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Systems Programming

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Operating Systems

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

#### Linux

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


#### Windows

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


#### macOS

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Compilers and Interpreters

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Compilers

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Interpreters

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Garbage Collection

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Networks

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Web Development

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Frontend

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Backend

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

#### Model-View-Controller

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


#### Microservices

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Web Applications Security

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Databases

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### SQL Databases

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### NoSQL Databases

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Concurrency and Parallelism

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Multithreading

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### SIMD

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Security

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |

### Cryptography

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


### Web Applications Security

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Blockchain

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## Testing

| Resource type | Progress |
| --- | --- |
| __Articles__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Books__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Courses__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| __Videos__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |
| | |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&amp;leftFill=%23ff0000) |


## See also
- [Teach Yourself Programming in Ten Years](http://norvig.com/21-days.html)
- [Teach Yourself Computer Science](https://teachyourselfcs.com/)
- [Become a Programmer, Motherfucker](http://programming-motherfucker.com/become.html)
- [Project Based Learning](https://github.com/tuvttran/project-based-learning)
- [Open Source Society University](https://github.com/open-source-society/computer-science)
- [Free Programming Books](https://github.com/vhf/free-programming-books)
- [Google Interview University](https://github.com/jwasham/google-interview-university)
- [Machine Learning for Software Engineers](https://github.com/ZuzooVn/machine-learning-for-software-engineers)
- [Awesome Compilers](http://aalhour.com/awesome-compilers)
- [What happens when...](https://github.com/alex/what-happens-when)
