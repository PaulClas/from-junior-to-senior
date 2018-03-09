# From junior to senior

<img src="https://cdn.rawgit.com/KolesnichenkoDS/from-junior-to-senior/master/resources/sicp.jpg"
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
vim resources/items.json

# render to markdown
cargo run -- resources/ -f json -F markdown -o README.md
```

If you want to suggest a resource, you can just [open an issue](https://github.com/KolesnichenkoDS/from-junior-to-senior/issues).

## Statistics

| Resource type | Progress |
| --- | --- |
| Articles | ![12](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=12&leftFill=%236699ff) |
| Books | ![25](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=25&leftFill=%236699ff) |
| Courses | ![11](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=11&leftFill=%236699ff) |
| Videos | ![100](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=100&leftFill=%236699ff) |
| __Total__ | ![14](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=14&leftFill=%236699ff) |

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
- [Programming Languages & Platforms](#programming-languages--platforms)
  - [JVM](#jvm)
    - [Java](#java)
    - [Kotlin](#kotlin)
    - [Scala](#scala)
  - [.NET](#net)
    - [C#](#c)
    - [F#](#f)
  - [Haskell](#haskell)
  - [Rust](#rust)
- [Programming Paradigms and Patterns](#programming-paradigms-and-patterns)
  - [Object-Oriented Programming](#object-oriented-programming)
  - [Functional Programming](#functional-programming)
  - [Logic Programming](#logic-programming)
  - [Actor Model](#actor-model)
  - [Inversion of Control](#inversion-of-control)
    - [Dependency Injection](#dependency-injection)
- [Memory Management and Garbage Collection](#memory-management-and-garbage-collection)
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
  - [Distributed Systems](#distributed-systems)
- [Security](#security)
  - [Cryptography](#cryptography)
  - [Web Applications Security](#web-applications-security)
- [Blockchain](#blockchain)
- [Testing](#testing)
- [Data Science](#data-science)
  - [Data Visualisation](#data-visualisation)
  - [Distributed Systems](#distributed-systems)
  - [Machine Learning](#machine-learning)


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

- :paperclip: __Cheatsheets__
  - :us: [Typeclassopedia](https://wiki.haskell.org/Typeclassopedia) by Brent Yorgey
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
  - [ ] :ru: [Дискретный анализ и теория вероятностей](https://yandexdataschool.ru/edu-process/courses/probability) by [School of Data Analysis](https://yandexdataschool.com/)

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





## Algorithms


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
  - [ ] :ru: [Алгоритмы и структуры данных](https://yandexdataschool.ru/edu-process/courses/algorithms) by [School of Data Analysis](https://yandexdataschool.com/)

</details>



### Complexity


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
  - [ ] :ru: [Сложность вычислений](https://yandexdataschool.ru/edu-process/courses/complexity) by [School of Data Analysis](https://yandexdataschool.com/)

</details>



## Programming Languages & Platforms


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![6](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=6&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![6](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=6&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>


</details>


### JVM


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

- :pencil: __Articles__
  - [ ] :us: [JVM Anatomy Park #2: Transparent Huge Pages](https://shipilev.net/jvm-anatomy-park/2-transparent-huge-pages) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #3: GC Design and Pauses](https://shipilev.net/jvm-anatomy-park/3-gc-design-and-pauses) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #4: TLAB Allocation](https://shipilev.net/jvm-anatomy-park/4-tlab-allocation) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #5: TLABs and Heap Parsebility](https://shipilev.net/jvm-anatomy-park/5-tlabs-and-heap-parsability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #6: New Object Stages](https://shipilev.net/jvm-anatomy-park/6-new-object-stages) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #7: Object Initialization Costs](https://shipilev.net/jvm-anatomy-park/7-initialization-costs) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #8: Local Variable Reachability](https://shipilev.net/jvm-anatomy-park/8-local-var-reachability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #9: JNI Critical and GC Locker](https://shipilev.net/jvm-anatomy-park/9-jni-critical-gclocker) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #10: String.intern()](https://shipilev.net/jvm-anatomy-park/10-string-intern) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #11: Moving GC and Locality](https://shipilev.net/jvm-anatomy-park/11-moving-gc-locality) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #12: Native Memory Tracking](https://shipilev.net/jvm-anatomy-park/12-native-memory-tracking) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #13: Integrational Barriers](https://shipilev.net/jvm-anatomy-park/13-intergenerational-barriers) by [Aleksey Shipilev](https://shipilev.net/)

</details>


#### Java


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

- :pencil: __Articles__
  - [ ] :us: [JVM Anatomy Park #1: Lock Coarsening and Loops](https://shipilev.net/jvm-anatomy-park/1-lock-coarsening-for-loops) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #8: Local Variable Reachability](https://shipilev.net/jvm-anatomy-park/8-local-var-reachability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #14: Constant Variables](https://shipilev.net/jvm-anatomy-park/14-constant-variables) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #15: Just-In-Time Constants](https://shipilev.net/jvm-anatomy-park/15-just-in-time-constants) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #16: Megamorphic Virtual Calls](https://shipilev.net/jvm-anatomy-park/16-megamorphic-virtual-calls) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #17: Trust Non-Static Final Fields](https://shipilev.net/jvm-anatomy-park/17-trust-nonstatic-final-fields) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #18: Scalar Replacement](https://shipilev.net/jvm-anatomy-park/18-scalar-replacement) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #19: Lock Elision](https://shipilev.net/jvm-anatomy-park/19-lock-elision) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #20: FPU Spills](https://shipilev.net/jvm-anatomy-park/20-fpu-spills) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [RAII in Java](http://www.yegor256.com/2017/08/08/raii-in-java.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Java 9: The Good, The Bad, and Private Interface Methods](http://www.yegor256.com/2017/10/03/java9.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Operator new() is Toxic](http://www.yegor256.com/2018/01/02/operator-new-is-toxic.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Constructors or Static Factory Methods?](http://www.yegor256.com/2017/11/14/static-factory-methods.html) by [Yegor Bugayenko](http://www.yegor256.com/)

</details>





### .NET


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![33](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=33&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![33](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=33&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [x] :us: [Concurrent Programming in .NET Core](http://www.dotnetcurry.com/dotnet/1360/concurrent-programming-dotnet-core) by Damir Arh
  - [ ] :us: [How does Object.GetType() really work?](http://tooslowexception.com/how-does-gettype-work/) by Konrad Kokosa
  - [ ] :us: [Zero Garbage Collector for .NET Core](http://tooslowexception.com/zero-garbage-collector-for-net-core/) by Konrad Kokosa
  - [ ] :us: [Zero Garbage Collector for .NET Core 2.1 and ASP.NET Core 2.1](http://tooslowexception.com/zero-garbage-collector-for-net-core-2-1-and-asp-net-core-2-1/) by Konrad Kokosa
  - [x] :us: [Allocation is cheap... until it is not](http://tooslowexception.com/allocation-is-cheap-until-it-is-not/) by Konrad Kokosa
  - [ ] :us: [Making .NET code less allocatey – Allocations and the Garbage Collector](https://blog.maartenballiauw.be/post/2016/10/19/making-net-code-less-allocatey-garbage-collector.html) by Maarten Balliauw

</details>




### Haskell


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

- :paperclip: __Cheatsheets__
  - :us: [Typeclassopedia](https://wiki.haskell.org/Typeclassopedia) by Brent Yorgey

</details>




## Programming Paradigms and Patterns


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


</details>


### Object-Oriented Programming


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

- :pencil: __Articles__
  - [ ] :us: [SRP is a Hoax](http://www.yegor256.com/2017/12/19/srp-is-hoax.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [DAO is Yet Another OOP Shame](http://www.yegor256.com/2017/12/05/data-access-object.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [ORM is an Offensive Anti-Pattern](http://www.yegor256.com/2014/12/01/orm-offensive-anti-pattern.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Getters/Setters. Evil. Period.](http://www.yegor256.com/2014/09/16/getters-and-setters-are-evil.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Operator new() is Toxic](http://www.yegor256.com/2018/01/02/operator-new-is-toxic.html) by [Yegor Bugayenko](http://www.yegor256.com/)
  - [ ] :us: [Constructors or Static Factory Methods?](http://www.yegor256.com/2017/11/14/static-factory-methods.html) by [Yegor Bugayenko](http://www.yegor256.com/)
- :book: __Books__
  - [ ] :us: [Design Patterns: Elements of Reusable Object-Oriented Software](https://www.amazon.com/Design-Patterns-Object-Oriented-Addison-Wesley-Professional-ebook/dp/B000SEIBB8) by Erich Gamma, Richard Helm, Ralph Johnson, John Vlissides

</details>



### Functional Programming


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

- :paperclip: __Cheatsheets__
  - :us: [Typeclassopedia](https://wiki.haskell.org/Typeclassopedia) by Brent Yorgey

</details>





### Inversion of Control


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

- :pencil: __Articles__
  - [ ] :us: [How Does Inversion of Control Really Work?](http://www.yegor256.com/2017/05/10/inversion-of-control.html) by [Yegor Bugayenko](http://www.yegor256.com/)

</details>



## Memory Management and Garbage Collection


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![6](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=6&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![6](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=6&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [ ] :us: [JVM Anatomy Park #2: Transparent Huge Pages](https://shipilev.net/jvm-anatomy-park/2-transparent-huge-pages) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #3: GC Design and Pauses](https://shipilev.net/jvm-anatomy-park/3-gc-design-and-pauses) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #4: TLAB Allocation](https://shipilev.net/jvm-anatomy-park/4-tlab-allocation) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #5: TLABs and Heap Parsebility](https://shipilev.net/jvm-anatomy-park/5-tlabs-and-heap-parsability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #6: New Object Stages](https://shipilev.net/jvm-anatomy-park/6-new-object-stages) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #7: Object Initialization Costs](https://shipilev.net/jvm-anatomy-park/7-initialization-costs) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #8: Local Variable Reachability](https://shipilev.net/jvm-anatomy-park/8-local-var-reachability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #9: JNI Critical and GC Locker](https://shipilev.net/jvm-anatomy-park/9-jni-critical-gclocker) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #10: String.intern()](https://shipilev.net/jvm-anatomy-park/10-string-intern) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #11: Moving GC and Locality](https://shipilev.net/jvm-anatomy-park/11-moving-gc-locality) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #13: Integrational Barriers](https://shipilev.net/jvm-anatomy-park/13-intergenerational-barriers) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [Zero Garbage Collector for .NET Core](http://tooslowexception.com/zero-garbage-collector-for-net-core/) by Konrad Kokosa
  - [ ] :us: [Zero Garbage Collector for .NET Core 2.1 and ASP.NET Core 2.1](http://tooslowexception.com/zero-garbage-collector-for-net-core-2-1-and-asp-net-core-2-1/) by Konrad Kokosa
  - [x] :us: [Allocation is cheap... until it is not](http://tooslowexception.com/allocation-is-cheap-until-it-is-not/) by Konrad Kokosa
  - [ ] :us: [Making .NET code less allocatey – Allocations and the Garbage Collector](https://blog.maartenballiauw.be/post/2016/10/19/making-net-code-less-allocatey-garbage-collector.html) by Maarten Balliauw

</details>









## Compilers and Interpreters


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


</details>


### Compilers


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

- :pencil: __Articles__
  - [ ] :us: [JVM Anatomy Park #1: Lock Coarsening and Loops](https://shipilev.net/jvm-anatomy-park/1-lock-coarsening-for-loops) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #8: Local Variable Reachability](https://shipilev.net/jvm-anatomy-park/8-local-var-reachability) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #14: Constant Variables](https://shipilev.net/jvm-anatomy-park/14-constant-variables) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #15: Just-In-Time Constants](https://shipilev.net/jvm-anatomy-park/15-just-in-time-constants) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #16: Megamorphic Virtual Calls](https://shipilev.net/jvm-anatomy-park/16-megamorphic-virtual-calls) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #17: Trust Non-Static Final Fields](https://shipilev.net/jvm-anatomy-park/17-trust-nonstatic-final-fields) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #18: Scalar Replacement](https://shipilev.net/jvm-anatomy-park/18-scalar-replacement) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #19: Lock Elision](https://shipilev.net/jvm-anatomy-park/19-lock-elision) by [Aleksey Shipilev](https://shipilev.net/)
  - [ ] :us: [JVM Anatomy Park #20: FPU Spills](https://shipilev.net/jvm-anatomy-park/20-fpu-spills) by [Aleksey Shipilev](https://shipilev.net/)

</details>










## Databases


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

- :pencil: __Articles__
  - [ ] :us: [ORM is an Offensive Anti-Pattern](http://www.yegor256.com/2014/12/01/orm-offensive-anti-pattern.html) by [Yegor Bugayenko](http://www.yegor256.com/)

</details>




## Concurrency and Parallelism


<details>
<summary>Statistics</summary>

| Resource type | Progress |
| --- | --- |
| Articles | ![50](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=50&leftFill=%236699ff) |
| Books | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Courses | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| Videos | ![0](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=0&leftFill=%236699ff) |
| __Total__ | ![33](http://www.yarntomato.com/percentbarmaker/button.php?barPosition=33&leftFill=%236699ff) |

</details>



<details>
<summary>Resources</summary>

- :pencil: __Articles__
  - [x] :us: [Concurrent Programming in .NET Core](http://www.dotnetcurry.com/dotnet/1360/concurrent-programming-dotnet-core) by Damir Arh

</details>


### Multithreading


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

- :pencil: __Articles__
  - [ ] :us: [JVM Anatomy Park #1: Lock Coarsening and Loops](https://shipilev.net/jvm-anatomy-park/1-lock-coarsening-for-loops) by [Aleksey Shipilev](https://shipilev.net/)
- :mortar_board: __Courses__
  - [ ] :ru: [Параллельные и распределённые вычисления](https://yandexdataschool.ru/edu-process/courses/parallel) by [School of Data Analysis](https://yandexdataschool.com/)

</details>




### Distributed Systems


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
  - [ ] :ru: [Параллельные и распределённые вычисления](https://yandexdataschool.ru/edu-process/courses/parallel) by [School of Data Analysis](https://yandexdataschool.com/)

</details>






## Testing


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

- :pencil: __Articles__
  - [ ] :us: [Single Statement Unit Tests](http://www.yegor256.com/2017/05/17/single-statement-unit-tests.html) by [Yegor Bugayenko](http://www.yegor256.com/)

</details>



## Data Science


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


</details>



### Distributed Systems


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
  - [ ] :ru: [Параллельные и распределённые вычисления](https://yandexdataschool.ru/edu-process/courses/parallel) by [School of Data Analysis](https://yandexdataschool.com/)

</details>



### Machine Learning


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
  - [ ] :ru: [Машинное обучение](https://yandexdataschool.ru/edu-process/courses/machine-learning) by [School of Data Analysis](https://yandexdataschool.com/)

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
