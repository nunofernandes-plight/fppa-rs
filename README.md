# FPPA (Field Programmable Photonic Array ) in Rust Project
Rust userspace runtime for controlling FPPA (Field Programmable Photonic Array ) PCIe devices with VFIO, IOMMU, MMIO, and zero-copy DMA.

# fppa-rs

**Rust userspace infrastructure for PCIe-attached programmable photonic hardware.**

`fppa-rs` is an experimental systems-software project exploring how a Rust userspace runtime can interact with a programmable photonic integrated circuit (PIC) through a PCIe interface.

The project focuses on the low-level boundary between **software-defined optical control and PCIe hardware**, with particular emphasis on:

- VFIO-based userspace device access
- IOMMU / IOMMUFD-based DMA isolation
- PCIe BAR and MMIO register access
- Zero-copy DMA memory
- Lock-free producer/consumer rings
- Hardware descriptors and ownership protocols
- Interrupt and completion handling
- Rust abstractions around unsafe hardware interfaces

The project is currently in an **early architectural / experimental stage**.

---

## Project Motivation

The project originated from an investigation into **high-radix programmable optical switching** and the software architecture required to control large programmable photonic meshes.

At the photonic layer, programmable silicon-photonic systems can expose large numbers of controllable optical elements, such as Mach-Zehnder interferometers and their associated actuators. A software control plane therefore needs to translate high-level optical configurations into deterministic hardware operations.

That leads progressively down the software stack:

```text
Optical Network
      │
      ▼
SDN / Network Scheduler
      │
      ▼
Photonic Topology / Configuration
      │
      ▼
FPPA Control Runtime
      │
      ▼
PCIe Device Interface
      │
      ├───────────────┐
      ▼               ▼
     MMIO            DMA
      │               │
      ▼               ▼
   BAR Registers   DMA Ring
          \         /
           \       /
            ▼     ▼
             IOMMU
               │
               ▼
              VFIO
               │
               ▼
             PCIe
               │
               ▼
          FPPA Hardware
```

`fppa-rs` concentrates on this **software-to-hardware boundary**.

---

## Architecture

The current architecture is intentionally divided into hardware access, memory management, and device-specific layers.

```text
src/
├── main.rs
│
├── vfio/
│   ├── mod.rs
│   ├── device.rs
│   ├── region.rs
│   └── irq.rs
│
├── iommu/
│   ├── mod.rs
│   ├── legacy_vfio.rs
│   └── iommufd.rs
│
├── dma/
│   ├── mod.rs
│   ├── allocation.rs
│   ├── ring.rs
│   └── descriptor.rs
│
├── mmio/
│   ├── mod.rs
│   └── bar.rs
│
└── hardware/
    ├── mod.rs
    └── fppa.rs
```

### Layer responsibilities

#### `vfio/`

Provides the userspace interface to the PCIe device through Linux VFIO mechanisms.

Responsibilities include:

- Device access
- VFIO region discovery
- PCIe device regions
- Interrupt/event handling

The VFIO layer should remain independent from the FPPA-specific device protocol.

---

#### `iommu/`

Provides the abstraction between host memory and device-visible DMA addresses.

Two implementation paths are planned:

```text
DmaMapper
   │
   ├── Legacy VFIO DMA mapping
   │
   └── IOMMUFD
```

A central design principle is maintaining the distinction between:

```text
CPU Virtual Address
        ≠
Physical Address
        ≠
IOVA
```

The device should consume an address explicitly established through the IOMMU mapping mechanism rather than assuming that a CPU pointer is a physical or DMA address.

---

#### `dma/`

Contains the data-plane memory infrastructure.

Responsibilities include:

- DMA allocation
- DMA descriptors
- Ring-buffer management
- Producer/consumer ownership
- DMA synchronization

The intended model is a zero-copy path in which data is written directly into DMA-capable memory rather than copied through intermediate buffers.

---

#### `mmio/`

Provides the register-access abstraction for PCIe BAR regions.

The layer is responsible for:

- BAR mappings
- Volatile register reads
- Volatile register writes
- Register-level access semantics

Device-specific register meanings should remain outside the generic MMIO implementation.

---

#### `hardware/`

Contains the FPPA-specific hardware abstraction.

This layer translates low-level mechanisms such as:

```text
BAR offsets
DMA descriptors
doorbells
interrupts
status registers
```

into higher-level FPPA operations.

The goal is that application code does not need to know that a particular operation corresponds to a specific BAR offset.

---

## PCIe Control and DMA Model

The project separates the two principal hardware interaction paths.

### Control plane — MMIO

```text
Rust Runtime
     │
     ▼
 VFIO Region
     │
     ▼
 PCIe BAR
     │
     ▼
Control / Status Registers
```

This path is intended for operations such as:

- device configuration
- status polling
- control commands
- ring configuration
- doorbell notifications

---

### Data plane — DMA

```text
Rust Runtime
     │
     ▼
DMA-capable memory
     │
     ▼
IOMMU
     │
     ▼
IOVA
     │
     ▼
PCIe DMA
     │
     ▼
FPPA Device
```

The ring-buffer design is intended to minimize unnecessary data movement while maintaining explicit ownership and synchronization between the host and device.

---

## Memory Safety Model

Hardware-facing Rust inevitably contains `unsafe` code.

The goal of `fppa-rs` is therefore **not to eliminate `unsafe`**, but to contain and document it.

Every hardware-facing unsafe boundary should answer five questions:

```text
1. Pointer validity
   └── Why is this address valid?

2. Ownership
   └── Who owns the memory?

3. Lifetime
   └── How long may the device access it?

4. Address space
   └── CPU virtual address, physical address, or IOVA?

5. Synchronization
   └── What prevents CPU/device races?
```

This is a core engineering principle of the project.

---

## MMIO and Memory Ordering

The project distinguishes between several concepts that are sometimes incorrectly combined in low-level examples:

```text
Compiler ordering
        │
        ▼
CPU memory ordering
        │
        ▼
MMIO access semantics
        │
        ▼
PCIe transaction ordering
        │
        ▼
Device-specific ordering requirements
```

Rust's volatile operations are used to express externally observable MMIO accesses.

Memory fences are treated separately as synchronization primitives and are not assumed to automatically establish every required PCIe or device-level ordering guarantee.

The final implementation will follow the ordering requirements of the target hardware and Linux interface rather than relying on assumptions from the initial prototype.

---

## DMA Ring Architecture

The intended ring-buffer model is:

```text
                 ┌─────────────────────────┐
                 │       DMA Ring          │
                 │                         │
                 │ [D0][D1][D2][D3] ...    │
                 │                         │
                 └─────────────────────────┘
                     ▲               ▲
                     │               │
                   HEAD            TAIL
                     │               │
                   Host            Device
```

Descriptors will explicitly represent ownership and lifecycle.

Conceptually:

```text
HOST_OWNED
    │
    ▼
READY
    │
    ▼
DEVICE_OWNED
    │
    ▼
COMPLETE
    │
    ▼
HOST_OWNED
```

The exact protocol will ultimately be defined by the target hardware interface.

---

## Why Rust?

The original project investigation compared C++20 and Rust for direct PCIe hardware interaction.

C++ offers mature systems-programming facilities and very direct control over hardware resources.

Rust adds another useful property: the ability to place strong ownership and lifetime constraints around the unsafe portions of the implementation.

The intended model is therefore:

```text
Safe Rust
   │
   ▼
Small, reviewed unsafe boundary
   │
   ▼
VFIO / IOMMU / MMIO / DMA
   │
   ▼
Hardware
```

Rather than allowing raw pointers and hardware addresses to propagate throughout the application, `fppa-rs` aims to encapsulate those mechanisms behind explicit abstractions.

---

## Development Philosophy

`fppa-rs` is being developed incrementally.

Each architectural layer should be introduced through a small, reviewable change rather than one large hardware-access implementation.

Planned progression:

```text
v0.1.0-alpha.1
      │
      ▼
Project architecture
      │
      ▼
VFIO foundation
      │
      ▼
BAR / MMIO abstraction
      │
      ▼
IOMMU abstraction
      │
      ▼
DMA allocation
      │
      ▼
DMA descriptors
      │
      ▼
DMA ring
      │
      ▼
IRQ / completion handling
      │
      ▼
FPPA hardware abstraction
```

Each stage should establish explicit invariants before the next hardware-facing layer is added.

---

## Current Status

**Version:** `v0.1.0-alpha.1`

**Status:** Early development / architecture phase

The current release establishes the initial project baseline.

The implementation is **not yet a production hardware driver** and should not be interpreted as validated against an actual iPronics FPPA PCIe device.

The project currently serves as an engineering framework for developing and validating the required userspace PCIe, IOMMU, MMIO, and DMA abstractions.

---

## Roadmap

### Phase 1 — Foundation

- [ ] Rust project structure
- [ ] Module boundaries
- [ ] Error model
- [ ] Basic testing infrastructure
- [ ] Documentation of hardware assumptions

### Phase 2 — PCIe / VFIO

- [ ] VFIO container handling
- [ ] VFIO group/device handling
- [ ] PCI configuration access
- [ ] Region discovery
- [ ] BAR mapping
- [ ] IRQ infrastructure

### Phase 3 — IOMMU

- [ ] DMA mapping abstraction
- [ ] Legacy VFIO implementation
- [ ] IOMMUFD implementation
- [ ] IOVA management
- [ ] Mapping/unmapping lifecycle

### Phase 4 — DMA

- [ ] DMA allocation
- [ ] Descriptor definitions
- [ ] Ring-buffer implementation
- [ ] Ownership protocol
- [ ] Memory-ordering model
- [ ] Completion handling

### Phase 5 — FPPA Hardware Layer

- [ ] FPPA register map
- [ ] Device initialization
- [ ] Ring configuration
- [ ] Doorbell mechanism
- [ ] Device status
- [ ] Hardware-specific command model

### Phase 6 — Validation

- [ ] Unit tests
- [ ] Mock PCIe/MMIO environment
- [ ] DMA/ring stress testing
- [ ] Error-path testing
- [ ] Hardware-in-the-loop validation
- [ ] Performance characterization

---

## Project Status and Scope

This repository is an **experimental systems-software project**.

Hardware register layouts, DMA descriptor formats, timing requirements, interrupt behavior, and device-specific protocols must be treated as hardware-contract assumptions until verified against authoritative device documentation or an actual test platform.

No claim is made here that the current abstractions represent the proprietary implementation of any particular commercial FPPA device.

---

## License

*License to be defined.*

---

## Origin

The project began as an exploration of high-radix optical circuit switching and programmable photonic architectures, progressing from optical-network concepts into the software/hardware interface required to control photonic hardware over PCIe.

`fppa-rs` is the continuation of that investigation at the systems-programming layer.
