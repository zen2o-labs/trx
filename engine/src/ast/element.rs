use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Id(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ShapeKind {
    // Basic Geometry
    Box,
    Rectangle,
    RoundedRectangle,
    Circle,
    Ellipse,
    Diamond,
    Triangle,
    Hexagon,
    Octagon,
    Parallelogram,
    Trapezoid,

    // Flowchart & UML
    Actor,
    Person,
    Component,
    Folder,
    Note,
    Envelope,
    Document,
    MultiDocument,
    Subroutine,
    Terminal,

    // IT, Infrastructure & Networking
    Server,
    Database,
    Cylinder,
    Storage,
    Tape,
    Cloud,
    Router,
    Switch,
    Firewall,
    LoadBalancer,
    Network,
    Cluster,

    // Devices & Hardware
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Printer,

    // Cloud Native & Software Architecture
    Container,
    Pod,
    Microservice,
    Function,
    Api,
    Endpoint,
    Queue,
    Topic,
    Message,
    Broker,
    Stack,

    // Security & Configuration
    Shield,
    Lock,
    Key,
    Vault,
    Gear,
    Certificate,

    // Schema & Data
    SqlTable,
    Package,
}
