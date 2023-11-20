## Project README

### 1) Problem Statement:   

#### Original Statement:
The project aims to explore and compare parallel programming features in Rust and C++, focusing on algorithms like mergesort, sum and root. It involves analyzing the performance of parallel vs. serial execution using Rust's Rayon and C++'s OpenMP.

#### POPL Angle:
- **POPL Involvement:** The POPL angle in this project lies in the application of parallel programming concepts, which are often discussed theoretically in Principles of Programming Languages (POPL) courses.
- **Uniqueness:** While parallel programming has been explored, our project differentiates itself by specifically comparing Rust and C++ in parallel computation.
### 2) Software Architecture:

#### Architecture Overview:
- **Rust Implementation:**
  - Leverages Rust's native threading capabilities and Rayon library.
  - Thread-based parallelism with shared-memory model.

- **C++ Implementation:**
  - Utilizes C++ threads and OpenMP for parallel execution.
  - Combines both thread-based and directive-based parallelism.

#### Reuse and Development:
- **Reused Components:**
  - Leveraged external libraries (Rayon for Rust, OpenMP for C++) for parallelism.
  - Incorporated standard libraries for data manipulation and random number generation.

- **Developed Components:**
  - Original implementations of parallel algorithms (mergesort, matrix multiplication).
  - Integration of parallel features specific to Rust and C++.

#### Testing Component:
- **Testing Approach:**
  - Local testing primarily during development.
  - External libraries (Rayon, OpenMP) have their own testing mechanisms.

- **Database Involvement:**
  - No database involvement; the focus is on parallel algorithm execution and performance analysis.

### 3) POPL Aspects:

#### Implementation Points:

1. **Thread Creation (Rust):**
   - **POPL Aspect:** Illustrates Rust's native thread creation for parallel execution.
   - **Experience:** Ensuring proper thread management and synchronization to prevent data race conditions was challenging. Rust's ownership model helped in avoiding common pitfalls.

2. **Mutex Usage (Rust):**
   - **POPL Aspect:** Demonstrates the application of mutexes to control thread access.
   - **Experience:** Implementing mutexes was crucial to prevent concurrent modification of shared data. Balancing data access and avoiding deadlocks required careful consideration.

3. **Parallel Sorting (Rayon):**
   - **POPL Aspect:** Applies the Rayon library for parallel sorting.
   - **Experience:** Integrating an external library for parallelism involved understanding and managing the library's internal parallelization strategy. It required adapting the algorithm to fit the library's paradigm.

4. **Thread Creation (C++):**
   - **POPL Aspect:** Demonstrates C++ thread creation for parallel execution.
   - **Experience:** Managing thread creation and synchronization in C++ posed challenges compared to Rust. Ensuring proper thread management was essential to prevent race conditions.

5. **Parallel Sorting (C++ Parallel Execution):**
   - **POPL Aspect:** Utilizes C++ parallel execution for sorting.
   - **Experience:** Ensuring proper parallelization in C++ involved addressing platform-specific considerations. Understanding the nuances of the parallel execution model in C++ was crucial.

6. **Data Race Prevention (Both):**
   - **POPL Aspect:** Emphasizes the importance of preventing data races by using locks or other synchronization mechanisms.
   - **Experience:** Ensuring data integrity across parallel threads was a common challenge. The experience involved identifying critical sections and applying synchronization techniques.

7. **Memory Model Awareness (Rust):**
   - **POPL Aspect:** Demonstrates Rust's ownership model, which ensures memory safety without the need for explicit locks.
   - **Experience:** Adapting to Rust's ownership model required a shift in mindset but provided advantages in preventing common memory-related issues.

8. **Task Parallelism (Both):**
   - **POPL Aspect:** Highlights the use of task-based parallelism for dividing work into independent tasks.
   - **Experience:** Balancing tasks and optimizing their distribution across threads presented a challenge in achieving efficient parallel execution.

9. **Load Balancing (Both):**
   - **POPL Aspect:** Addresses the distribution of work to threads to ensure uniform load balancing.
   - **Experience:** Optimizing load balancing involved considerations of the input data's characteristics and dynamic adjustments to thread assignments.

10. **Scalability (Both):**
    - **POPL Aspect:** Focuses on the system's ability to efficiently scale with an increasing number of threads.
    - **Experience:** Achieving optimal scalability required experimentation with thread counts and assessing performance under various workloads.


### 4) Results and Testing:

#### Test Conducted:
- Mergesort and matrix multiplication algorithms implemented in both Rust and C++.
- Performance measured for serial and parallel execution using time benchmarks.

#### Datasets and Benchmarks:
- **Dataset:** Randomly generated arrays of integers or doubles.
- **Benchmarks:** Execution time measured for each algorithm and parallel implementation.

#### Validation:
- Ensure consistency with the initial problem statement by validating:
  - Performance improvements in parallel execution.
  - Proper functioning of implemented algorithms.

### 5) Potential for Future Work:

#### Potential Future Directions:
- **Optimization Strategies:** Investigate additional avenues for optimizing parallel algorithms.
- **Diversify Algorithms:** Implement and compare a broader spectrum of parallel algorithms.
- **Incorporate Advanced POPL Concepts:** Integrate sophisticated POPL concepts to elevate the level of parallelism.

### Rust Codes:

#### Prerequisites:
1. Rust installed on your machine. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

#### Steps:

##### For "parallel_merge_sort.rs":
1. Open a terminal.
2. Navigate to the directory containing "parallel_merge_sort.rs".
3. Run the command:
   ```bash
   cargo run --release
   ```

##### For "parallel_sort_rayon.rs":
1. Open a terminal.
2. Navigate to the directory containing "parallel_sort_rayon.rs".
3. Run the command:
   ```bash
   cargo run --release
   ```

##### For "parallel_vector_creation.rs":
1. Open a terminal.
2. Navigate to the directory containing "parallel_vector_creation.rs".
3. Run the command:
   ```bash
   cargo run --release
   ```

### C++ Codes:

#### Prerequisites:
1. C++ compiler installed on your machine. If not, you can install GCC or another suitable compiler.

#### Steps:

##### For "merge_sort.cpp":
1. Open a terminal.
2. Navigate to the directory containing "merge_sort.cpp".
3. Compile the code:
   ```bash
   g++ merge_sort.cpp -o merge_sort -std=c++11
   ```
4. Run the compiled executable:
   ```bash
   ./merge_sort
   ```

##### For "parallel_merge_sort.cpp":
1. Open a terminal.
2. Navigate to the directory containing "parallel_merge_sort.cpp".
3. Compile the code:
   ```bash
   g++ parallel_merge_sort.cpp -o parallel_merge_sort -std=c++11 -pthread
   ```
4. Run the compiled executable:
   ```bash
   ./parallel_merge_sort
   ```

##### For "parallel_sort.cpp":
1. Open a terminal.
2. Navigate to the directory containing "parallel_sort.cpp".
3. Compile the code:
   ```bash
   g++ parallel_sort.cpp -o parallel_sort -std=c++11 -pthread
   ```
4. Run the compiled executable:
   ```bash
   ./parallel_sort
   ```

These instructions assume a Linux terminal environment. For Windows, the commands and compilation steps may differ. Adjust accordingly based on your operating system.
