# Thanatos Architecture Documentation

## 1. High-Level Overview

Thanatos is an application that provides a set of tools and algorithms for simulating, storing, and exploring generations of Conway's Game of Life. \
At its core is the **MFRAC** (Memoized Forward-Reachability Attractor-Collapsing) algorithm, designed to efficiently manage large-scale simulation and storage of Conway configurations while identifying
and collapsing recurrent configurations.

**Purpose:**

- Efficiently compute and analyze Conway generations.
- Accelerate generation processing through *MFRAC*.
- Provide GUI-based visualization and analysis of the explored state space.
- Support optional CLI access for automation or scripting.
- Integrate profiling and benchmarking for developers and users.

**Scope:**

- Core *TMFRAC* algorithm loop (Conway -> *MFRAC* -> repeat).
- GUI for visualization and statistical analysis.
- Profiling and benchmarking tools (flamegraphs, memory tracking).
- Optional CLI tools for automation or batch runs.

---

## 2. Core Components / Modules

### `src/ui/`

- Main GUI framework.
- Grid visualization.
- Interactive controls for running simulations and viewing results.
- Analysis tools interaction.

### `src/conway/`

- Conway Game of Life simulation engine.
- Stateless generation step through.

### `src/mfrac/`

- Memoized Forward-Reachability Attractor-Collapsing algorithm implementation.
- Core computational engine interfacing with the simulation module.

### `src/profilers/`

- Integrated profiling tools for CPU, memory, and stack usage.
- Generates artefacts like flamegraphs, heap snapshots, and runtime logs.

### `src/types/`

- Common types used throughout the application.

### `src/bin/`

- CLI tooling such as headless profilers.

### `benches/`

- Benchmarking suite for performance-critical sections of the program.

### `tests/`

- Testing suite.

### `artifacts/`

- Runtime-generated outputs (SVG flamegraphs, logs, memory dumps).
- Ignored by Git, grouped by type:

### `docs/`

- Architecture and design notes, diagrams, documented profiling results.
- Captures decisions, trade-offs, and long-term reasoning.
- Brain dumps of ideas for the project.

---

## 3. Data Flow / Execution Flow for Runs

1. **Simulation Step:** Conway simulation generates next configuration.
2. ***MFRAC* Processing:** processes current configuration through the *MFRAC* pipeline. View *MFRAC* algorithm specification for details.
3. **(Optional) UI Update:** sends current configuration to the UI thread.
4. **(Optional) Profiling Hooks:** performance data collected during each loop, optionally output to `artifacts/`.
5. Repeat loop until simulation stop condition.

**Notes:**

- Simulation and UI are on different threads. The UI lagging behind the state of a simulation running uncapped is to be expected.

---

## 4. Design Decisions

- **Modularity**: each module of the application is contained and only interacted through strict internal APIs.
- **Multi-Threading**: simulation runs on its own thread to ensure maximum efficiency.
- **UI as Second-Class**: the simulation display is a secondary concern. Simulation speed is sacred.
- **Integrated Profiling and Benchmarking**: emphasis on performance monitoring during development and usage.
- **Artefact Folder Structure**: separates runtime outputs from source code.

---

## 5. Infrastructure

- TBD

---

## 6. Future Considerations / TODOs

- Extended CLI tooling for batch simulations and automated analysis.
- Improve profiling granularity (per-generation, per-module).
- Configurable benchmarking for development and users to allow for gauging individual machine capabilities.
- Expand GUI analytics (charts, histograms, collapse rates, attractor visualization, configuration/shape heatmaps).
- Refactor profiling tooling into a modular crate for easier maintenance.

---

## 7. Notes on What This Doc Contains

- **Includes:** architecture overview, module responsibilities, high-level flow, design rationale.
- **Excludes:** implementation details, full function bodies, temporary experimental notes.