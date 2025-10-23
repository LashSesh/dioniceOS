//! # 5D System Framework
//!
//! A deterministic framework for simulating coupled dynamical systems in five dimensions.
//! Based on the mathematical specification in 5D_System_Framework.pdf
//!
//! ## Modules
//! - `state`: 5D state vectors and operations
//! - `coupling`: Coupling matrix and interaction types
//! - `dynamics`: Vector field and evolution operators
//! - `integration`: Numerical integration schemes
//! - `stability`: Stability analysis and Lyapunov exponents
//! - `projection`: Dimension reduction and visualization
//! - `template`: Domain-specific instantiation templates
//! - `export`: Data export in CSV and JSON formats
//! - `validation`: Reference solutions for testing

pub mod state;
pub mod coupling;
pub mod dynamics;
pub mod integration;
pub mod stability;
pub mod projection;
pub mod template;
pub mod export;
pub mod validation;
pub mod ensemble;

pub use state::State5D;
pub use coupling::{CouplingMatrix, CouplingType};
pub use dynamics::{SystemParameters, VectorField};
pub use integration::Integrator;
pub use template::Template;
