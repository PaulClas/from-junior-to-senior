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

## Statistics

| Resource type | Progress |
| --- | --- |
| Articles | ![60](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=60&leftFill=%236699ff) |
| Books | ![33](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=33&leftFill=%236699ff) |
| Courses | ![25](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=25&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![46](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=46&leftFill=%236699ff) |

</details>


## Table of contents

- [Calculus](#calculus)
- [Functional Analysis](#functional-analysis)
- [Geometry](#geometry)
- [Logic](#logic)
  - [Linear Logic](#linear-logic)
- [Algebra](#algebra)
  - [Linear Algebra](#linear-algebra)
- [Category Theory](#category-theory)
- [Combinatorics and Probability](#combinatorics-and-probability)
- [Discrete Mathematics](#discrete-mathematics)
- [Type Theory](#type-theory)
  - [Linear Type Theory](#linear-type-theory)
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


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| Books | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [x] :us: [An Intuitive Introduction to Limits](https://betterexplained.com/articles/an-intuitive-introduction-to-limits) by [Better Explained](https://betterexplained.com/)
- :book: __Books__
  - [x] :ru: [Краткий курс математического анализа](http://nuclphys.sinp.msu.ru/mathan/) by Lev Kudryavtsev

</details>



## Functional Analysis


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![50](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=50&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [ ] :us: [An Interactive Guide to the Fourier Transform](https://betterexplained.com/articles/an-interactive-guide-to-the-fourier-transform) by [Better Explained](https://betterexplained.com/)
- :mortar_board: __Courses__
  - [x] :us: [Analytic Geometry](https://www.khanacademy.org/math/geometry-home/analytic-geometry-topic) by [Khan Academy](https://www.khanacademy.org/)

</details>




## Logic


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![66](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=66&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![75](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=75&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [x] :us: [Sequent Calculus](https://ncatlab.org/nlab/show/sequent+calculus) by [nLab](https://ncatlab.org/)

</details>


### Linear Logic


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![50](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=50&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![66](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=66&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [x] :us: [Linear Logic](https://ncatlab.org/nlab/show/linear+logic) by [nLab](https://ncatlab.org/)
  - [ ] :ru: [Линейная логика (реферат по истории математики)](https://www.docme.ru/doc/925474/linejnaya-logika) by Alexandr Kharitonov
- :movie_camera: __Videos__
  - [x] :us: [Linear and Dependent Types 1](https://youtu.be/5i3YDgQyIwE) by Neel Krishnaswami

</details>



## Algebra


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :book: __Books__
  - [ ] :us: [Algebra: Chapter 0](https://www.amazon.com/Algebra-Chapter-Graduate-Studies-Mathematics/dp/0821847813) by Paolo Aluffi
- :mortar_board: __Courses__
  - [ ] :us: [Algebra I](https://ocw.mit.edu/courses/mathematics/18-701-algebra-i-fall-2010/) by [MIT OpenCourseWare](https://ocw.mit.edu/)
  - [ ] :us: [Algebra II](https://ocw.mit.edu/courses/mathematics/18-702-algebra-ii-spring-2011/) by [MIT OpenCourseWare](https://ocw.mit.edu/)

</details>



## Category Theory


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :book: __Books__
  - [ ] :us: [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/) by [Bartosz Milewski](https://bartoszmilewski.com/)

</details>



## Combinatorics and Probability


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :mortar_board: __Courses__
  - [ ] :us: [Basic Theoretical Probability](https://www.khanacademy.org/math/statistics-probability/probability-library) by [Khan Academy](https://www.khanacademy.org/)

</details>




## Type Theory


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>


</details>


### Linear Type Theory


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :movie_camera: __Videos__
  - [x] :us: [Linear and Dependent Types 1](https://youtu.be/5i3YDgQyIwE) by Neel Krishnaswami

</details>



## Lambda Calculus


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>


</details>


### Linear Lambda Calculus


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :movie_camera: __Videos__
  - [x] :us: [Linear and Dependent Types 1](https://youtu.be/5i3YDgQyIwE) by Neel Krishnaswami

</details>









































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
