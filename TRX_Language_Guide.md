# TRX Language: The Data-Oriented Diagramming DSL

TRX is a high-performance, Domain Specific Language (DSL) designed for zero-overhead, mechanically sympathetic graph construction and data visualization. It compiles instantly to Wasm, SVG, and Interactive Web environments.

## Core Concepts

TRX is built around a few first-class primitives: **Diagrams**, **Packets**, **States**, and **Charts**.

---

## 1. Diagrams
Diagrams are the primary way to define architectures and networks.

### Syntax
```trx
# Diagram Name
[node] identifier [Label] { properties }
src -> dst : "Connection Label"
```

### Examples
- **Basic Connection**: `A -> B`
- **Labeling**: `Client[Browser] -> Server[NodeJS] : "HTTP GET"`
- **Arrow Types**: 
  - `->` Standard connection
  - `>>` Async/Queue connection
  - `==` Tight binding/Database connection

---

## 2. Packets
Packets allow for bit-accurate memory layout and network payload definition.

### Syntax
```trx
packet Name [attributes] {
    start..end : FieldName [type: T, style: S]
    constraint: FieldName <= value
}
```

---

## 3. State Machines
Define discrete state transitions with clarity.

### Syntax
```trx
state Name {
    [*] -> InitialState
    StateA -> StateB : @trigger
}
```

---

## 4. XY Charts
Bind real-time telemetry or data sources to visual plots.

### Syntax
```trx
xy Name {
    x_axis: "Label"
    y_axis: "Label"
    data: @source::path
}
```

---

## 5. SQL Schema Tables
Visualize relational schemas directly.

### Syntax
```trx
sqltable Users {
    PK id: uuid
    email: varchar(255)
}

sqltable Orders {
    PK id: uuid
    FK user_id: uuid -> Users
    status: varchar(50)
}
```

---

## 6. Math, Logic & Scenarios
TRX supports inline math for dynamic property assignment:
```trx
let base_padding = 10
node A { padding: base_padding * 2 }
```

Filter complex diagrams down to a single scenario view using:
```trx
[scenario: "happy_path"]
```

---

## 7. Primitives & Styling
Nodes can be assigned complex geometric primitive shapes using the `shape` attribute via the `StyleBuffer`.

**Available Shapes:** `circle`, `ellipse`, `diamond`, `hexagon`, `cloud`, `cylinder` / `database`, `parallelogram`, `triangle`, `rounded`, `box` (default).

```trx
# Hexagon geometry with a raw HEX fill pattern
Gateway [shape: hexagon, fill: "#ff0000"] { width: 150 }
```

---

## Technical Summary
- **Zero-Copy WebAssembly Bridge**: Memory layouts are designed to be cache-friendly, allowing direct pointer-based execution from the host JS environment.
- **Glassmorphic & Flat Rendering**: Styles map to 4-byte RGBA arrays.
- **Dual-Engine Layout**: Supports both Force-directed (mesh physics) and Layered/Sugiyama (workflow/topological) layouts.
- **Universal Compilation**: One TRX file generates JSON, SVG, and Interactive Web (ARIA) reports automatically.
