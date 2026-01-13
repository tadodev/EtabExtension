# CLAUDE.md

> **Context document for AI agents working on the E2K Parser project**

## Project Overview

**EtabExtension** is a local-first desktop application for parsing, analyzing, and managing ETABS E2K structural analysis files. Built with Tauri (Rust backend) + React (TypeScript frontend), inspired by the architecture of [LaReview](https://github.com/puemos/lareview).

### Core Purpose
- Parse ETABS E2K files into structured data (Points, Frames,Shell, Materials, Loads,Analysis,Preferences)
- Provide 3D visualization of structural models
- Enable version control (Git integration) for E2K files
- Export professional post-processing reports (mode, displacement, drift, shear&spandrel stress, story force, core axial force,...etc.) (PDF, Excel) 
- Validate structural models against engineering standards

---

## Architecture Philosophy

### 1. **Local-First Design**
- All data stored in SQLite (`~/Library/Application Support/EtabExtension/db.sqlite` on widnows)
- No cloud dependencies or backend servers
- User owns and controls their engineering data
- Works offline by default
- For later phases consider to add AI agent assistance for design suggestions, code reviews, 
and report generation both cloud and local

### 2. **Heavy Lifting in Rust, Lightweight React**
```
┌─────────────────────────────────────┐
│         Rust Backend (Tauri)        │
│  • E2K Parsing (nom crate)          │
│  • Syntax Highlighting (syntect)    │
│  • Database (sea-orm)              │
│  • Git Operations (git2)            │
│  • PDF/Excel Export                 │
│  • Structural Validation            │
└──────────────┬──────────────────────┘
               │ Tauri IPC
               │ (JSON over WebSocket)
               ▼
┌─────────────────────────────────────┐
│      React Frontend (TypeScript)    │
│  • UI Components (shadcn/ui)        │
│  • 3D Viewer (React Three Fiber)    │
│  • Charts (ECharts)                 │
│  • Editor (Monaco)                  │
│  • State Management (Zustand)       │
│  • Data Tables (TanStack Table)     │
└─────────────────────────────────────┘
```

**Key Principle**: Rust does computation-heavy work (parsing, validation, export), React handles display and user interaction.

### 3. **Technology Stack**

#### Rust Backend
- **Tauri 2.x** - Desktop app framework
- **tauri-plugin-fs,
  tauri-plugin-dialog,
  tauri-plugin-shell** - File system, dialogs, shell commands
- **Tokio 1.4x** - Async runtime
- **nom 8.x** - Parser combinator for E2K format
- **regex 1.1x** - Regular expressions for parsing
- **syntect 5.x** - Syntax highlighting for E2K files
- **polars 0.5x** - DataFrame for data manipulation and calculation
- **SeaORM 2.x** - Async ORM with code-first approach
- **sea-orm-migration** - Database migrations
- **git2 0.20** - Git integration
- **serde/serde_json** - Serialization
- **printpdf 0.8** - PDF generation
- **rust_xlsxwriter 0.9x** - Excel export
- **encoding_rs 0.8** - Handle Windows-1251 encoding
- **thiserror 2.x** - Error handling
- **anyhow 1.x** - Generic error handling
- **chrono 0.4** - Date/time handling
- **validator 0.2** - Data validation
- **log 0.4 + tracing 0.1** - Logging
- **uuid 1.1x** - Unique ID generation
- **rayon 1.1x** - Parallel processing
- **dashmap 6.1x** - Concurrent HashMap
- **notify 8.x** - File system watching (optional)
- **rhai 1.2x** - Scripting engine (optional)

#### React Frontend
- **React 19** - UI framework
- **TypeScript 5.8** - Type safety
- **Vite 7** - Build tool
- **Tailwind CSS 4** - Styling
- **shadcn/ui** - Component library
- **Monaco Editor** - Code editor
- **React Three Fiber** - 3D visualization
- **ECharts** - Data visualization
- **TanStack Query** - Data fetching
- **TanStack Table** - Data tables
- **Zustand** - State management
- **Zustand** - State management
- **Immer** - Immutable updates
- **React Hook Form + Zod** - Forms & validation
- **date-fns** - Date formatting
- **Sonner** - Toast notifications
- **cmdk** - Command palette
- **Lucide React** - Icons
---

## Project Structure

```
etabextension/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/            # Tauri commands (Rust → JS bridge)
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs        # E2K parsing commands
│   │   │   ├── database.rs      # Database operations
│   │   │   ├── git.rs           # Git version control
│   │   │   └── export.rs        # PDF/Excel export
│   │   ├── entities/            # SeaORM entity models
│   │   │   ├── mod.rs
│   │   │   ├── project.rs       # Project entity
│   │   │   ├── point.rs         # Point entity
│   │   │   ├── frame.rs         # Frame entity
│   │   │   ├── material.rs      # Material entity
│   │   │   └── load.rs          # Load entity
│   │   ├── db/
│   │   │   ├── mod.rs           # Database connection & service
│   │   │   └── migrator.rs      # Migration runner
│   │   ├── services/
│   │   │   ├── git_service.rs   # Git operations
│   │   │   ├── highlight.rs     # Syntect highlighting
│   │   │   └── validator.rs     # Structural validation
│   │   ├── lib.rs               # Main library
│   │   └── main.rs              # Entry point
│   ├── migration/               # SeaORM migrations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── m20240101_000001_create_tables.rs
│   │   │   └── m20240102_000001_add_validation.rs
│   │   └── Cargo.toml
│   ├── Cargo.toml
│   ├── build.rs
│   └── tauri.conf.json          # Tauri configuration
│
├── crates/                       # Workspace crates
│   ├── e2k-parser/              # E2K Parser Library (Rust)
│   │   ├── src/
│   │   │   ├── models/          # Data structures
│   │   │   │   ├── mod.rs
│   │   │   │   ├── point.rs     # Point (node/joint)
│   │   │   │   ├── frame.rs     # Frame (beam/column)
│   │   │   │   ├── shell.rs     # Shell elements
│   │   │   │   ├── material.rs  # Material properties
│   │   │   │   └── load.rs      # Load cases
│   │   │   ├── parser/          # Parser implementation
│   │   │   │   ├── mod.rs
│   │   │   │   ├── point_parser.rs
│   │   │   │   ├── frame_parser.rs
│   │   │   │   ├── shell_parser.rs
│   │   │   │   └── section_parser.rs
│   │   │   ├── encoding.rs      # Windows-1251 handling
│   │   │   ├── error.rs         # Error types
│   │   │   └── lib.rs           # Public API
│   │   ├── tests/
│   │   │   ├── fixtures/        # Sample E2K files
│   │   │   └── integration_tests.rs
│   │   └── Cargo.toml
│   │
│   ├── database/                # Database layer (optional)
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── git-service/             # Git service (optional)
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
│
├── src/                          # React frontend
│   ├── components/
│   │   ├── ui/                  # shadcn/ui components
│   │   ├── Editor/              # Monaco Editor wrapper
│   │   │   └── MonacoEditor.tsx
│   │   ├── Viewer3D/            # Three.js 3D viewer
│   │   │   ├── Scene.tsx
│   │   │   ├── StructureModel.tsx
│   │   │   └── Controls.tsx
│   │   ├── Charts/              # ECharts visualizations
│   │   │   ├── ForceChart.tsx
│   │   │   └── DisplacementChart.tsx
│   │   ├── DataTable/           # TanStack Table
│   │   │   ├── DataTable.tsx
│   │   │   └── columns/
│   │   ├── Git/                 # Version control UI
│   │   │   ├── GitHistory.tsx
│   │   │   └── DiffViewer.tsx
│   │   └── Export/              # Export dialogs
│   │       └── ExportDialog.tsx
│   ├── hooks/                   # Custom React hooks
│   │   ├── useE2KParser.ts
│   │   ├── useProject.ts
│   │   └── useGit.ts
│   ├── services/                # Tauri command wrappers
│   │   ├── parser.ts
│   │   ├── database.ts
│   │   ├── git.ts
│   │   └── export.ts
│   ├── stores/                  # Zustand stores
│   │   ├── projectStore.ts
│   │   ├── editorStore.ts
│   │   ├── viewerStore.ts
│   │   └── uiStore.ts
│   ├── types/                   # TypeScript types
│   │   ├── e2k.ts              # Mirror Rust models
│   │   ├── project.ts
│   │   └── git.ts
│   ├── styles/
│   │   └── index.css           # Tailwind + custom styles
│   ├── App.tsx                 # Main app component
│   └── main.tsx                # React entry point
│
├── migrations/                  # Database migrations
│   ├── 001_initial.sql
│   ├── 002_add_validation.sql
│   └── 003_add_git_tracking.sql
│
├── docs/
│   ├── ARCHITECTURE.md
│   ├── DEVELOPMENT.md
│   └── E2K_FORMAT.md           # E2K file format specification
│
├── scripts/
│   ├── build_macos_app.sh
│   └── setup_dev.sh
│
├── Cargo.toml                  # Workspace configuration
├── package.json                # NPM dependencies
├── tsconfig.json               # TypeScript config
├── vite.config.ts              # Vite config
├── tailwind.config.js          # Tailwind config
├── components.json             # shadcn/ui config
└── rust-toolchain.toml         # Rust version pinning
```

---

## Core Data Models

### E2K File Structure

E2K files are text-based structural analysis files with sections like:

```
$ POINT COORDINATES
1  0.0  0.0  0.0
2  3.0  0.0  0.0
3  6.0  0.0  0.0

$ FRAME CONNECTIVITY
F1  1  2  W14X68  A992
F2  2  3  W14X68  A992

$ MATERIAL PROPERTIES
A992  STEEL  200000000000  0.3

$ LOAD CASES
DEAD  FRAME  F1  -5000
LIVE  POINT  2  -10000
```

### Rust Data Models

```rust
// src-tauri/src/entities/point.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "points")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// src-tauri/src/entities/frame.rs
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "frames")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub project_id: String,
    pub point_i: String,
    pub point_j: String,
    pub section: String,
    pub material: String,
}

// ... similar pattern for Material, Load, Project
```

### TypeScript Types (Mirror Rust)

```typescript
// src/types/e2k.ts

export interface Point {
  id: string;
  x: number;
  y: number;
  z: number;
}

export interface Frame {
  id: string;
  pointI: string;  // Note: camelCase in TS
  pointJ: string;
  section: string;
  material: string;
}

export interface E2KModel {
  points: Point[];
  frames: Frame[];
  materials: Material[];
  loads: Load[];
}
```

---

## Development Rules

### 1. **Rust Code Guidelines**

#### Parsing
- Use `nom` for parsing E2K sections
- Handle Windows-1251 encoding (common in E2K files)
- Return `Result<T, Error>` for all fallible operations
- Write unit tests for each parser function

```rust
// Example parser structure
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    sequence::tuple,
    IResult,
};

fn parse_point_line(input: &str) -> IResult<&str, Point> {
    let (input, (id, _, x, _, y, _, z)) = tuple((
        digit1,
        space1,
        double,
        space1,
        double,
        space1,
        double,
    ))(input)?;
    
    Ok((input, Point {
        id: id.to_string(),
        x, y, z,
    }))
}
```

#### Error Handling
- Use `thiserror` for custom errors
- Never use `unwrap()` or `expect()` in production code
- Use `?` operator for error propagation
- Log errors with `log::error!`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E2KError {
    #[error("Invalid point definition: {0}")]
    InvalidPoint(String),
    
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

#### Tauri Commands
- Always async for non-blocking operations
- Return `Result<T, String>` (String for error messages)
- Use `#[tauri::command]` macro
- Keep commands thin - delegate to services

```rust
#[tauri::command]
pub async fn parse_e2k_file(file_path: String) -> Result<E2KModel, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    e2k_parser::parse(&content)
        .map_err(|e| e.to_string())
}
```

### 2. **TypeScript Code Guidelines**

#### Type Safety
- Never use `any` - use `unknown` if type is unclear
- Define interfaces for all data structures
- Use `Zod` for runtime validation of user input
- Mirror Rust types exactly

```typescript
// Bad
function parseData(data: any) { ... }

// Good
function parseData(data: unknown): E2KModel {
  const validated = E2KModelSchema.parse(data);
  return validated;
}
```

#### Tauri Integration
- Wrap all `invoke` calls in service functions
- Handle errors with proper error types
- Use TanStack Query for data fetching

```typescript
// src/services/parser.ts
import { invoke } from "@tauri-apps/api/core";
import type { E2KModel } from "@/types/e2k";

export async function parseE2KFile(filePath: string): Promise<E2KModel> {
  try {
    return await invoke<E2KModel>("parse_e2k_file", { filePath });
  } catch (error) {
    throw new Error(`Failed to parse E2K file: ${error}`);
  }
}

// Use in component
import { useQuery } from "@tanstack/react-query";

function useE2KFile(filePath: string | null) {
  return useQuery({
    queryKey: ["e2k", filePath],
    queryFn: () => parseE2KFile(filePath!),
    enabled: !!filePath,
  });
}
```

#### Component Guidelines
- Use functional components with hooks
- Extract complex logic to custom hooks
- Use shadcn/ui components for consistency
- Keep components under 300 lines

```typescript
// Bad - too much logic in component
function MyComponent() {
  const [data, setData] = useState(null);
  useEffect(() => {
    fetch('/api/data').then(r => r.json()).then(setData);
  }, []);
  // ... 500 lines of JSX
}

// Good - logic in custom hook
function MyComponent() {
  const { data, isLoading } = useProjectData();
  
  if (isLoading) return <LoadingSpinner />;
  return <DataDisplay data={data} />;
}
```

### 3. **Git Integration Rules**

- Store git metadata in SQLite (not in memory)
- Track E2K file changes automatically on save
- Generate structured diffs (not just text diffs)
- Use `git2` crate (not shell commands)

```rust
// Compare two E2K versions structurally
pub struct E2KDiff {
    pub points_added: Vec<Point>,
    pub points_removed: Vec<Point>,
    pub points_modified: Vec<(Point, Point)>,  // (old, new)
    // ... same for frames, materials, loads
}
```

### 4. **Export Rules**

#### PDF Export
- Use company template system (user-configurable)
- Include: header, logo, tables, diagrams, footer
- Generate 3D diagrams using headless rendering
- Page numbers and table of contents

#### Excel Export
- Multiple sheets: Summary, Points, Frames, Materials, Loads
- Format headers with colors
- Auto-fit columns
- Freeze header rows
- Add charts/graphs where appropriate

### 5. **Database Schema Rules**

- **Use SeaORM entities (code-first approach)**
- **Auto-generate migrations** from entity definitions
- Use transactions for multi-table operations
- Index foreign keys for performance
- Leverage SeaORM relations for joins

**Entity Definition Pattern:**
```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub file_path: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::point::Entity")]
    Points,
}

impl ActiveModelBehavior for ActiveModel {}
```

**Migration Generation:**
```bash
# Install SeaORM CLI
cargo install sea-orm-cli

# Generate migration from entities
sea-orm-cli migrate generate create_tables

# Run migrations
sea-orm-cli migrate up
```

**Database Operations:**
```rust
// Create
let project = project::ActiveModel {
    id: Set(uuid::Uuid::new_v4().to_string()),
    name: Set("My Project".to_string()),
    ..Default::default()
}.insert(&db).await?;

// Read
let project = Project::find_by_id("project-id")
    .one(&db)
    .await?;

// Update
let mut project: project::ActiveModel = project.into();
project.name = Set("Updated Name".to_string());
project.update(&db).await?;

// Delete
project.delete(&db).await?;

// Relations
let project_with_points = Project::find_by_id("project-id")
    .find_with_related(Point)
    .all(&db)
    .await?;
```

###  **Parallel Processing with Rayon**

**When to Use:**
- Parsing large E2K files (>1000 elements)
- Batch processing multiple files
- Structural calculations on large datasets

**Rules:**
```rust
// ✅ DO: Use rayon for CPU-bound operations
use rayon::prelude::*;

fn parse_e2k_sections(sections: &[String]) -> Vec<ParsedSection> {
    sections
        .par_iter()  // Parallel iterator
        .map(|s| parse_section(s))
        .collect()
}

// ❌ DON'T: Use rayon for I/O operations (use tokio instead)
// Wrong: Reading files in parallel with rayon
sections.par_iter().map(|path| std::fs::read(path))

// Right: Use tokio for async I/O
let futures: Vec<_> = sections.iter().map(|path| {
    tokio::fs::read(path)
}).collect();
let results = futures::future::join_all(futures).await;
```

**Performance Rule:**
- Only use `.par_iter()` when processing >100 items
- Profile first - parallel isn't always faster for small datasets
- Avoid nested parallelism (rayon inside rayon)

---

###  **Concurrent Caching with DashMap**

**When to Use:**
- Material properties lookup (frequently accessed, rarely changed)
- Section property cache
- Parse result caching

**Rules:**
```rust
use dashmap::DashMap;
use once_cell::sync::Lazy;

// ✅ DO: Use Lazy static for global caches
static MATERIAL_CACHE: Lazy<DashMap<String, Material>> = 
    Lazy::new(|| DashMap::new());

pub fn get_material(name: &str) -> Option<Material> {
    MATERIAL_CACHE.get(name).map(|m| m.value().clone())
}

pub fn cache_material(name: String, material: Material) {
    MATERIAL_CACHE.insert(name, material);
}

// ✅ DO: Clear cache when project changes
pub fn clear_material_cache() {
    MATERIAL_CACHE.clear();
}

// ❌ DON'T: Use DashMap for data that changes frequently
// Wrong: Using DashMap for point coordinates (they change often)
// Right: Use DashMap for material properties (stable reference data)
```

**Cache Invalidation Rule:**
- Always clear cache when:
    - Loading a new E2K file
    - Switching projects
    - Modifying material definitions

---

###  **File Watching with Notify**

**When to Use:**
- Detect external E2K file modifications (by ETABS/SAP2000)
- Auto-reload on file change
- Sync with external tools

**Rules:**
```rust
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};
use tokio::sync::mpsc;

// ✅ DO: Use async channel for file events
pub async fn watch_e2k_file(
    file_path: PathBuf,
    tx: mpsc::Sender<FileEvent>
) -> Result<RecommendedWatcher, Error> {
    let (notify_tx, mut notify_rx) = mpsc::channel(100);
    
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = notify_tx.blocking_send(event);
        }
    })?;
    
    watcher.watch(&file_path, RecursiveMode::NonRecursive)?;
    
    // Forward events to Tauri
    tokio::spawn(async move {
        while let Some(event) = notify_rx.recv().await {
            let _ = tx.send(FileEvent::from(event)).await;
        }
    });
    
    Ok(watcher)
}

// ❌ DON'T: Watch entire directories recursively (performance hit)
// Wrong:
watcher.watch("/Users/name/Projects", RecursiveMode::Recursive)?;

// Right:
watcher.watch("/Users/name/Projects/model.e2k", RecursiveMode::NonRecursive)?;
```

**File Watch Rule:**
- Only watch the specific E2K file currently open
- Stop watching when project closes
- Debounce rapid changes (wait 500ms after last event)
- Show UI notification when file changes externally

---

###  **Polars DataFrame Operations**

**When to Use:**
- Structural analysis calculations
- Load combinations
- Force/moment calculations
- Statistical analysis of results

**Rules:**
```rust
use polars::prelude::*;

// ✅ DO: Use lazy evaluation for performance
pub fn calculate_stresses(
    frames: &[Frame],
    forces: &[Force]
) -> Result<DataFrame, PolarsError> {
    let df = DataFrame::new(vec![
        Series::new("frame_id", frames.iter().map(|f| &f.id).collect::<Vec<_>>()),
        Series::new("area", frames.iter().map(|f| f.area).collect::<Vec<_>>()),
        Series::new("force", forces.iter().map(|f| f.value).collect::<Vec<_>>()),
    ])?;
    
    // Use lazy API for complex operations
    df.lazy()
        .with_column(
            (col("force") / col("area")).alias("stress")
        )
        .filter(col("stress").gt(lit(250.0))) // Only high stress
        .collect()
}

// ✅ DO: Use Series for single column operations
pub fn get_max_displacement(displacements: &[f64]) -> f64 {
    let series = Series::new("disp", displacements);
    series.max().unwrap()
}

// ❌ DON'T: Use Polars for small datasets (<100 rows)
// Wrong: Using Polars for 10 points
// Right: Use Vec<Point> and standard Rust iterators

// ❌ DON'T: Convert between Polars and Vec repeatedly
// Wrong:
for point in points {
    let df = create_df_from_point(point); // Creating DF per point
    process_df(df);
}

// Right:
let df = create_df_from_points(&points); // Single DF
process_df(df);
```

**Polars Performance Rules:**
- Use lazy evaluation (`.lazy()`) for operations on >1000 rows
- Batch operations instead of per-row processing
- Use `.par_apply()` for expensive per-row calculations
- Only convert to/from Vec when necessary

---

###  **UUID Generation**

**When to Use:**
- Creating new projects
- Generating unique entity IDs
- Git commit references

**Rules:**
```rust
use uuid::Uuid;

// ✅ DO: Use v4 (random) for most cases
pub fn create_project_id() -> String {
    Uuid::new_v4().to_string()
}

// ✅ DO: Use v5 (deterministic) when you need reproducibility
pub fn create_entity_id(project_id: &str, entity_name: &str) -> String {
    let namespace = Uuid::parse_str(project_id).unwrap();
    Uuid::new_v5(&namespace, entity_name.as_bytes()).to_string()
}

// ✅ DO: Store UUIDs as strings in database
#[derive(DeriveEntityModel)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,  // UUID as string
    // ...
}

// ❌ DON'T: Generate UUIDs on frontend
// Wrong:
// TypeScript: const id = crypto.randomUUID();

// Right:
// Rust: let id = Uuid::new_v4().to_string();
```

**UUID Storage Rule:**
- Always generate UUIDs in Rust backend
- Store as `String` in SeaORM models
- Use hyphenated format: `550e8400-e29b-41d4-a716-446655440000`

---

###  **Validator for Input Validation**

**When to Use:**
- Validating E2K parsed data
- User input from forms
- API command parameters

**Rules:**
```rust
use validator::{Validate, ValidationError};

// ✅ DO: Derive Validate on models
#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Point {
    #[validate(length(min = 1, max = 50))]
    pub id: String,
    
    #[validate(range(min = -1000.0, max = 1000.0))]
    pub x: f64,
    
    #[validate(range(min = -1000.0, max = 1000.0))]
    pub y: f64,
    
    #[validate(range(min = 0.0, max = 500.0))]
    pub z: f64,
}

// ✅ DO: Validate before saving to database
pub async fn save_point(point: Point, db: &Database) -> Result<(), Error> {
    point.validate()
        .map_err(|e| Error::Validation(e))?;
    
    db.insert_point(point).await?;
    Ok(())
}

// ✅ DO: Custom validation for domain rules
#[derive(Debug, Validate)]
pub struct Frame {
    #[validate(custom = "validate_different_points")]
    pub point_i: String,
    pub point_j: String,
}

fn validate_different_points(frame: &Frame) -> Result<(), ValidationError> {
    if frame.point_i == frame.point_j {
        return Err(ValidationError::new("points_must_differ"));
    }
    Ok(())
}
```

**Validation Rules:**
- Validate all parsed E2K data before database insertion
- Return clear error messages to user
- Use custom validators for engineering rules (e.g., "moment > 0")

---

###  **Tracing for Structured Logging**

**When to Use:**
- Performance profiling
- Debugging complex operations
- Production monitoring

**Rules:**
```rust
use tracing::{info, warn, error, debug, instrument, span, Level};

// ✅ DO: Use instrument macro for function tracing
#[instrument(skip(db))]
pub async fn parse_e2k_file(
    file_path: &str,
    db: &Database
) -> Result<E2KModel, Error> {
    info!("Starting E2K parse");
    
    let content = tokio::fs::read_to_string(file_path).await?;
    debug!(size = content.len(), "File loaded");
    
    let model = parse_content(&content)?;
    info!(
        points = model.points.len(),
        frames = model.frames.len(),
        "Parse complete"
    );
    
    Ok(model)
}

// ✅ DO: Use spans for complex operations
pub fn calculate_forces(frames: &[Frame]) -> Vec<Force> {
    let _span = span!(Level::INFO, "calculate_forces", 
        frame_count = frames.len()).entered();
    
    // Complex calculation
    let forces = expensive_calculation(frames);
    
    info!("Force calculation complete");
    forces
}

// ✅ DO: Initialize tracing in main
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    // ... rest of app
}

// Use: RUST_LOG=debug cargo run
// Use: RUST_LOG=etabextension=trace cargo run (module-specific)
```

**Tracing vs Log Rule:**
- Use `log::info!` for simple messages
- Use `tracing::info!` with structured fields for searchable data
- Use `#[instrument]` on all Tauri commands
- Use `span!` for timing critical sections

---

###  **Toast Notifications with Sonner**

**When to Use:**
- User feedback for async operations
- Success/error states
- Progress updates

**Rules:**
```typescript
import { toast } from "sonner";

// ✅ DO: Use promise toast for async operations
async function parseFile(path: string) {
    toast.promise(
        parseE2KFile(path),
        {
            loading: 'Parsing E2K file...',
            success: (data) => `Parsed ${data.points.length} points successfully`,
            error: (err) => `Failed to parse: ${err.message}`,
        }
    );
}

// ✅ DO: Add actions to error toasts
toast.error('Failed to save project', {
    description: error.message,
    action: {
        label: 'Retry',
        onClick: () => retrySave(),
    },
});

// ✅ DO: Use duration based on importance
toast.success('Saved', { duration: 2000 }); // Quick feedback
toast.warning('Large file detected', { duration: 5000 }); // Important warning
toast.error('Critical error', { duration: Infinity }); // Stays until dismissed

// ❌ DON'T: Toast every state change
// Wrong:
onChange={() => {
    setState(value);
    toast.info('State changed'); // Too noisy
}}

// Right: Only toast user-initiated actions or async results
```

**Toast Rules:**
- Max 1 toast per user action
- Use `promise` toast for all async operations
- Error toasts should have "Retry" action when possible
- Success toasts: 2s duration
- Warning toasts: 5s duration
- Error toasts: Stay until dismissed

---

###  **Command Palette with cmdk**

**When to Use:**
- Power user shortcuts
- Quick access to features
- Discoverability

**Rules:**
```typescript
import { Command } from "cmdk";
import { useHotkeys } from "@mantine/hooks"; // or similar

// ✅ DO: Group commands logically
function CommandPalette() {
    const [open, setOpen] = useState(false);
    
    useHotkeys([['mod+K', () => setOpen(true)]]);
    
    return (
        <Command open={open} onOpenChange={setOpen}>
            <CommandInput placeholder="Type a command..." />
            <CommandList>
                <CommandGroup heading="File">
                    <CommandItem
                        onSelect={() => openFile()}
                        keywords={["open", "load", "import"]}
                    >
                        <FileIcon />
                        Open E2K File
                        <CommandShortcut>⌘O</CommandShortcut>
                    </CommandItem>
                    <CommandItem onSelect={() => saveProject()}>
                        <SaveIcon />
                        Save Project
                        <CommandShortcut>⌘S</CommandShortcut>
                    </CommandItem>
                </CommandGroup>
                
                <CommandGroup heading="View">
                    <CommandItem onSelect={() => toggleView('3d')}>
                        Toggle 3D View
                    </CommandItem>
                </CommandGroup>
                
                <CommandGroup heading="Tools">
                    <CommandItem onSelect={() => validateModel()}>
                        Validate Model
                        <CommandShortcut>⌘⇧V</CommandShortcut>
                    </CommandItem>
                </CommandGroup>
            </CommandList>
        </Command>
    );
}

// ✅ DO: Add keywords for searchability
<CommandItem keywords={["open", "load", "import", "file"]}>

// ✅ DO: Show keyboard shortcuts
<CommandShortcut>⌘K</CommandShortcut>
```

**Command Palette Rules:**
- Trigger with `Ctrl+K` (Windows/Linux) or `⌘K` (Mac)
- Group commands by category
- Show keyboard shortcuts
- Add keywords for better search
- Keep command list under 20 items per group

---

###  **Immer for Zustand State Updates**

**When to Use:**
- Complex nested state updates
- Array/object mutations in Zustand stores

**Rules:**
```typescript
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

// ✅ DO: Use immer middleware for complex state
interface ProjectState {
    projects: Project[];
    currentProject: Project | null;
    selectedPoints: string[];
    
    addPoint: (point: Point) => void;
    updatePoint: (id: string, updates: Partial<Point>) => void;
    togglePointSelection: (id: string) => void;
}

export const useProjectStore = create<ProjectState>()(
    immer((set) => ({
        projects: [],
        currentProject: null,
        selectedPoints: [],
        
        // ✅ With immer: Direct mutations
        addPoint: (point) => set((state) => {
            state.currentProject?.points.push(point);
        }),
        
        updatePoint: (id, updates) => set((state) => {
            const point = state.currentProject?.points.find(p => p.id === id);
            if (point) {
                Object.assign(point, updates);
            }
        }),
        
        togglePointSelection: (id) => set((state) => {
            const index = state.selectedPoints.indexOf(id);
            if (index > -1) {
                state.selectedPoints.splice(index, 1);
            } else {
                state.selectedPoints.push(id);
            }
        }),
    }))
);

// ❌ Without immer: Manual immutable updates (verbose)
addPoint: (point) => set((state) => ({
    currentProject: state.currentProject ? {
        ...state.currentProject,
        points: [...state.currentProject.points, point],
    } : null,
})),
```

**Immer Rules:**
- Always use immer middleware for stores with nested objects/arrays
- Mutate state directly inside `set()` - immer handles immutability
- Don't mix immutable patterns with immer (pick one style)
- Use for: arrays, nested objects, complex updates
- Skip for: simple primitive updates

---

##  Testing Rules

### Rayon Testing
```rust
#[test]
fn test_parallel_parsing() {
    let sections = generate_test_sections(1000);
    
    // Test sequential
    let start = Instant::now();
    let seq_results = sections.iter()
        .map(|s| parse_section(s))
        .collect::<Vec<_>>();
    let seq_time = start.elapsed();
    
    // Test parallel
    let start = Instant::now();
    let par_results = sections.par_iter()
        .map(|s| parse_section(s))
        .collect::<Vec<_>>();
    let par_time = start.elapsed();
    
    assert_eq!(seq_results, par_results);
    assert!(par_time < seq_time); // Parallel should be faster
}
```

### Polars Testing
```rust
#[test]
fn test_stress_calculation() {
    let df = DataFrame::new(vec![
        Series::new("force", &[1000.0, 2000.0, 1500.0]),
        Series::new("area", &[100.0, 150.0, 120.0]),
    ]).unwrap();
    
    let result = df.lazy()
        .with_column((col("force") / col("area")).alias("stress"))
        .collect()
        .unwrap();
    
    let stress = result.column("stress").unwrap();
    assert_eq!(stress.f64().unwrap().get(0), Some(10.0));
}
```

###  **UI/UX Guidelines**

- Light/Dark mode by System (engineering preference)
- Keyboard shortcuts for common actions
- Loading states for all async operations
- Error boundaries for crash recovery
- Tooltips for technical terms
- Responsive design (min width: 1024px)

---

## Key Features Implementation

### 1. E2K Parser
**Status**: 🔴 Not Started

**Implementation**:
- Parse E2K sections using `nom`
- Handle Windows-1251 encoding
- Validate data structures
- Generate structured `E2KModel`

### 2. 3D Visualization
**Status**: 🟢 Implemented (Demo)

**Current**: Basic Three.js demo with rotating cube
**Needed**: Render actual E2K structure (points as spheres, frames as lines)

### 3. Git Integration
**Status**: 🔴 Not Started

**Needed**:
- Initialize git repo for projects
- Auto-commit on save
- View commit history
- Compare versions with structural diff

### 4. PDF Export
**Status**: 🔴 Not Started

**Needed**:
- Template system (header, footer, logo)
- Render tables and diagrams
- Multi-page support
- Professional styling

### 5. Excel Export
**Status**: 🔴 Not Started

**Needed**:
- Multi-sheet workbook
- Formatted headers
- Charts/graphs
- Data validation

---

## Common Patterns

### 1. Database Service Pattern (SeaORM)

```rust
// src-tauri/src/db/mod.rs
use sea_orm::*;

pub struct Database {
    pub conn: DatabaseConnection,
}

impl Database {
    pub async fn new(db_path: PathBuf) -> Result<Self, DbErr> {
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
        let conn = sea_orm::Database::connect(&db_url).await?;
        
        // Run migrations
        migration::Migrator::up(&conn, None).await?;
        
        Ok(Self { conn })
    }
    
    pub async fn create_project(
        &self,
        name: String,
        file_path: Option<String>,
    ) -> Result<project::Model, DbErr> {
        use crate::entities::project;
        
        let project = project::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            name: Set(name),
            file_path: Set(file_path),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        };
        
        project.insert(&self.conn).await
    }
    
    pub async fn get_project_with_data(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectWithData>, DbErr> {
        let project = Project::find_by_id(project_id)
            .one(&self.conn)
            .await?;
        
        let Some(project) = project else {
            return Ok(None);
        };
        
        // Load related entities
        let points = project
            .find_related(Point)
            .all(&self.conn)
            .await?;
        
        let frames = project
            .find_related(Frame)
            .all(&self.conn)
            .await?;
        
        Ok(Some(ProjectWithData {
            project,
            points,
            frames,
        }))
    }
}
```

### 2. Tauri Command Pattern

```rust
// Rust side
use tauri::State;

#[tauri::command]
pub async fn create_project(
    name: String,
    file_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<project::Model, String> {
    let db = state.db.lock().await;
    
    db.create_project(name, file_path)
        .await
        .map_err(|e| e.to_string())
}

// TypeScript side
export async function createProject(
    name: string,
    filePath?: string
): Promise<Project> {
  return await invoke<Project>("create_project", { name, filePath });
}
```

### 3. State Management Pattern

```typescript
// Zustand store
interface ProjectState {
  currentProject: E2KModel | null;
  isLoading: boolean;
  error: string | null;
  
  loadProject: (filePath: string) => Promise<void>;
  clearProject: () => void;
}

export const useProjectStore = create<ProjectState>((set) => ({
  currentProject: null,
  isLoading: false,
  error: null,
  
  loadProject: async (filePath) => {
    set({ isLoading: true, error: null });
    try {
      const model = await parseE2KFile(filePath);
      set({ currentProject: model, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },
  
  clearProject: () => set({ currentProject: null, error: null }),
}));
```

### 3. Error Display Pattern

```typescript
// Always show user-friendly errors
try {
  await doRiskyOperation();
} catch (error) {
  toast.error(`Failed to save: ${error instanceof Error ? error.message : 'Unknown error'}`);
  log.error('Full error:', error);
}
```

---

## Testing Strategy

### Rust Tests
- Unit tests for each parser function
- Integration tests with real E2K files
- Property-based testing for validation
- SeaORM entity tests with in-memory SQLite

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_point() {
        let input = "1  0.0  5.0  3.5";
        let result = parse_point_line(input);
        assert!(result.is_ok());
        
        let (_, point) = result.unwrap();
        assert_eq!(point.id, "1");
        assert_eq!(point.x, 0.0);
    }
    
    #[tokio::test]
    async fn test_create_project() {
        // Use in-memory SQLite for testing
        let db = Database::new(":memory:".into()).await.unwrap();
        
        let project = db.create_project(
            "Test Project".to_string(),
            None,
        ).await.unwrap();
        
        assert_eq!(project.name, "Test Project");
    }
}
```

### React Tests
- Component tests with React Testing Library
- Hook tests with `@testing-library/react-hooks`
- E2E tests for critical workflows

---

## Performance Requirements

- E2K parsing: < 100ms for files up to 10,000 elements
- 3D rendering: 60 FPS for models up to 5,000 elements
- Database queries: < 50ms for typical operations
- UI interactions: < 16ms (60 FPS)
- PDF generation: < 5s for 20-page report
- Excel export: < 2s for 10,000 rows

---

## Logging & Debugging

### Rust Logging
```rust
use log::{info, warn, error, debug};

info!("Starting E2K parser");
debug!("Parsing line: {}", line);
warn!("Duplicate point ID: {}", id);
error!("Failed to parse: {}", err);
```

**Run with logging**:
```bash
RUST_LOG=debug cargo run          # All debug logs
RUST_LOG=e2k_parser=trace cargo run  # Module-specific
```

### TypeScript Logging
```typescript
console.log('[App] Loading project:', projectId);
console.warn('[Parser] Invalid data detected');
console.error('[Git] Failed to commit:', error);
```

---

## Current Status & Next Steps

### ✅ Completed
- Basic Tauri + React setup
- shadcn/ui integration
- Monaco Editor demo
- Three.js demo
- ECharts demo
- TanStack Table demo

### 🚧 In Progress
- Project structure organization
- Documentation

### 📋 Priority Queue
1. **E2K Parser** - Core functionality
2. **SQLite Integration** - Data persistence
3. **Git Integration** - Version control
4. **PDF Export** - Reports
5. **Excel Export** - Data sharing

---

## Contact & Resources

- **Repository**: (Not yet public)
- **Inspiration**: [LaReview](https://github.com/puemos/lareview)
- **Tech Docs**: See `/docs` directory
- **E2K Format**: See `/docs/E2K_FORMAT.md`

---

## For AI Agents

When working on this project:

1. **Always preserve the local-first architecture** - no cloud dependencies
2. **Do heavy lifting in Rust** - parsing, validation, export
3. **Keep React lightweight** - display and interaction only
4. **Use SeaORM for database operations** - code-first, type-safe
5. **Match TypeScript types to Rust structs** - maintain consistency
6. **Use proper error handling** - never `unwrap()` in production Rust, never `any` in TypeScript
7. **Follow the component structure** - don't create new top-level directories
8. **Test critical paths** - parser, database operations, Git integration
9. **Document complex logic** - especially E2K format parsing
10. **Consider performance** - this is for production engineering workflows
11. **Use SeaORM migrations** - generate from entities, never write SQL manually
12. **Leverage SeaORM relations** - use `find_with_related` instead of manual joins

When suggesting code changes:
- Show the full file path
- Indicate if creating new file or modifying existing
- Explain the reasoning behind architectural decisions
- Consider impact on other parts of the system
- Provide type-safe implementations
- Include error handling
- Add inline comments for complex logic
- For database changes, define SeaORM entities first, then generate migrations
- Use SeaORM's Active Record pattern for CRUD operations
- Leverage compile-time checks from SeaORM's derive macros

---

**Last Updated**: January 13, 2026
**Version**: 0.1.0 (Pre-release)
**Status**: Active Development
**ORM**: SeaORM 1.1 (Code-First Approach)