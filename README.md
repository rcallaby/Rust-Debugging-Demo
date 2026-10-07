# Rust Debugging Demo

**Learn to debug real Rust programs by fixing deliberately broken code.**

This repository is a collection of hands-on debugging assignments designed for students learning Rust. Each assignment presents a realistic, purposefully error-prone program filled with common pitfalls that beginners and intermediate learners frequently encounter. Your job is to diagnose the problems, understand *why* they occur, and produce a clean, idiomatic solution.

The goal is not just to make the code compile — it is to build strong debugging instincts, deepen your understanding of Rust’s ownership and type system, and practice writing robust, idiomatic code.

---

## Learning Objectives

By working through the assignments in this repository you will:

- Diagnose and fix ownership, borrowing, and lifetime errors
- Recognize and eliminate common anti-patterns (excessive cloning, `unwrap()` abuse, sentinel values, etc.)
- Improve error handling with `Option` and `Result`
- Write more idiomatic Rust (prefer `&str`, slices, proper ownership transfer, etc.)
- Read and act on compiler error messages effectively
- Refactor broken code into clear, maintainable solutions
- Document your debugging process

---

## Repository Structure

```
rust-debugging-gym/
├── README.md
├── assignments/
│   ├── 01-cursed-quest-log/
│   │   ├── README.md                 ← Assignment instructions
│   │   ├── src/
│   │   │   └── main.rs               ← The broken (cursed) code
│   │   └── solution/                 ← Reference solution (for instructors or self-check)
│   ├── 02-nebula-express-cargo-tracker/
│   │   ├── README.md
│   │   ├── src/
│   │   │   └── main.rs
│   │   └── solution/
│   ├── 03-...                        ← Future assignments
│   └── ...
├── common/                           ← Shared utilities or helper crates (if needed)
└── CONTRIBUTING.md
```

Each assignment folder contains:
- Clear instructions and learning goals
- A deliberately broken program
- Expected behavior description
- A reference solution (intended for instructors or self-verification)

---

## Getting Started

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-debugging-demo
   ```

2. **Choose an assignment**
   ```bash
   cd assignments/01-cursed-quest-log
   # or
   cd assignments/02-nebula-express-cargo-tracker
   ```

3. **Read the assignment README** carefully — it explains the story, the intended behavior, and what you need to deliver.

4. **Try to compile and run the broken code**
   ```bash
   cargo run
   ```
   Observe the compiler errors and/or runtime panics.

5. **Debug and fix** the program until it compiles cleanly and behaves correctly.

6. **Document your work** (most assignments ask for a short debug log explaining the errors you found and how you fixed them).

---

## Current Assignments

| #  | Name                              | Theme                                      | Focus Areas                                                                 | Difficulty    |
|----|-----------------------------------|--------------------------------------------|-----------------------------------------------------------------------------|---------------|
| 01 | Cursed Quest Log                  | Fantasy adventure inventory & quest system | Ownership, borrowing, lifetimes, `String`/`&str`, `unwrap`, sentinels, cloning | Intermediate |
| 02 | Nebula Express Cargo Tracker      | Interstellar freighter cargo management    | Same core 10 error categories in a completely new domain                    | Intermediate |
| 03 | *(Coming soon)*                   |                                            |                                                                             |               |

Both current assignments intentionally contain **the same 10 common Rust error categories**, but in completely different problem domains. This allows students to practice recognizing the same underlying issues in new contexts.

---

## How Assignments Are Designed

Every assignment follows the same philosophy:

- The starting code is **intentionally broken** with multiple realistic errors
- Errors are drawn from the most common mistakes students and developers make when learning Rust
- The program has a clear, interesting purpose (so fixing it feels rewarding)
- Students must both **make it work** and **make it idiomatic**
- A short reflection / debug log is usually required so learners articulate *what* went wrong and *why*

---

## For Instructors

- Reference solutions are provided in each assignment’s `solution/` folder.
- You can hide or remove the solution folder when distributing the repo to students.
- Assignments are independent — students can complete them in any order.
- Both existing assignments target the same core set of 10 error categories, making them useful for repeated practice or comparison.
- Feel free to adapt the stories, add tests, or create auto-graders on top of the exercises.

---

## Contributing

We welcome new debugging assignments!  

If you would like to contribute:

1. Look at an existing assignment for structure and tone.
2. Create a new folder under `assignments/`.
3. Include a clear student-facing README, the broken source code, and a reference solution.
4. Open a pull request.

Please make sure new exercises focus on **debugging skills** rather than pure feature implementation.

---

## License

This repository is licensed under the MIT License. See `LICENSE` for details.
 be ever in your favor.
```
