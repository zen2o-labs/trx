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

## 5. Math & Logic
TRX supports inline math for dynamic property assignment:
```trx
let base_padding = 10
node A { padding: base_padding * 2 }
```

---

## Technical Summary
- **Mechanical Sympathy**: Memory layouts are designed to be cache-friendly.
- **Glassmorphic Rendering**: The default visual style is neon-accented transparency.
- **Universal Compilation**: One TRX file generates JSON, SVG, and HTML reports.
