# Linear Regression Model in Rust

# Introduction
This project is a simple "linear regression model" it was built using Rust and the Burn library. The goal  was to get a train a model that will predicts `y = 2x + 1` based on generated data.

Setup & Installation
1. Install Rust & Rust Rover
First, I installed Rust.
Then, I installed the RustRover IDE.
Create a New Project name it.

I ran:
cargo new linear_regression_model  
cd linear_regression_model  
Modify Cargo.toml

I replaced the default Cargo.toml code with the one provided.
Testing the Initial Setup

Before adding the required files, I tried running the default main.rs to see if it printed "Hello, World!".
It didn’t work at first because I removed some required package configurations.
I added them back and was able to run "Hello, World!" successfully.
Creating Additional Files

I created train.rs, model.rs, and evaluate.rs.
I didn’t know about these files at first, but Copilot AI helped guide me.
Building the Project

Finally, I ran:
cargo build  
This made sure everything was set up correctly. but i faced error that resulted not being able to the the final result that are supposed to be displayed.  

5. Run the project  
   cargo run
- Unfortinantly i could not run cargo Run because i had errors form cargo buld that forten me to contiun with cargo run.

Challenges Faced
- Setting up Burn correctly: Some dependencies had to be fixed for version compatibility.
- Tensor operations: Had to adjust how tensors are created and updated.
- Pushing to GitHub: Git branch issues when trying to push changes. before that my laptop did not have the specif VCS so i option for the terminal to be able to commit the code to git
  # Errors while running the code
  - Incorrect imports: Had to update import paths for the Burn library.
  - Tensor initialization issues: Fixed issues related to missing device arguments.
  - Optimizer issues: Adam optimizer required an explicit learning rate.
  - Git push errors: Renamed the branch from `master` to `main` to fix push conflicts.

# Resources Used
- [Rust Documentation](https://doc.rust-lang.org/)
- [Burn Library Docs](https://docs.rs/burn/0.16.0/burn/)
- [GitHub Guide](https://docs.github.com/en/get-started)
- AI Copilot assistant

# What I Learned
- How to use Rust for AI/ML
- How to debug tensor operations
- How to properly manage GitHub repositories


This was a fun project to learn how to use **Rust** for simple AI models.

 # Submission 
 Submission

The complete source code is available on GitHub at https://....name and username.

The Cargo.toml file remains unchanged.

This README.md file documents the entire process.

The project was submitted via Blackboard LMS
