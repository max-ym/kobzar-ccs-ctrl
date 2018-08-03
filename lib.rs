#![feature(fused)]
use std::collections::BTreeMap;
use std::rc::Rc;

/// Basic elements to make up the network operation environment.
pub mod base;

/// Module implementing transactions.
mod trans;

pub use base::*;
pub use trans::*;

/// Master controller of CCS network.
pub struct Master {
}

/// Object hierarchy of the network.
pub struct ObjectHierarchy {
}

/// Interface hierarchy of the network.
pub struct InterfaceHierarchy {

    /// Name of the hierarchy node.
    name    : Rc<String>,

    /// Interfaces at current hierarchy level.
    ints    : Rc<Interface>,

    /// Tree of subpackages.
    subs    : BTreeMap<Rc<String>, InterfaceHierarchy>,
}


/// Source of interfaces. When some yet unknown interface is requested,
/// Master controller
/// calls for InterfaceLoader-s to look up for given interface and load
/// the interface if it gets found.
pub trait InterfaceLoader {

    /// Look for given interface in this source. All found interfaces
    /// are collected into single structure. Master can decide which
    /// of found interfaces to load into the network.
    fn find_interface(&mut self);

    /// Look for given package with interfaces.
    fn find_package(&mut self);
}

/// Source of interface implementers. When network needs to load new
/// object that implements specified interfaces, master calls for
/// ImplementerLoader to find required objects and load them.
pub trait ImplementerLoader {

    /// Look for given interface implementers in this source. All found
    /// objects are collected into single structure. Master can decide
    /// which of found objects to load into the network.
    fn find_implementer(&mut self);
}

impl Master {

    /// Apply changes of the transaction.
    pub fn apply_transaction(&mut self) {
        unimplemented!()
    }

    /// Object hierarchy of the network.
    pub fn objects(&self) {
        unimplemented!()
    }

    /// Interface hierarchy of the network.
    pub fn interfaces(&self) {
        unimplemented!()
    }

    /// Make access to the object mutable.
    pub fn make_object_mut(&mut self) {
        unimplemented!()
    }

    /// Make access to the service mutable.
    pub fn make_service_mut(&mut self) {
        unimplemented!()
    }

    /// All interface loaders currently registered.
    pub fn interface_loaders(&self) {
        unimplemented!()
    }

    /// All implementer loaders currently registered.
    pub fn implementer_loaders(&self) {
        unimplemented!()
    }

    /// Add new interface loader.
    pub fn add_interface_loader<L: InterfaceLoader>(&mut self, loader: L) {
        unimplemented!()
    }

    /// Add new implementer loader.
    pub fn add_implementer_loader<L: ImplementerLoader>(&mut self, loader: L) {
        unimplemented!()
    }
}

impl ObjectHierarchy {

    /// Object at the current level of hierarchy.
    pub fn current_object(&self) {
        unimplemented!()
    }

    /// All public objects one step lower in hierarchy.
    pub fn public_sub_objects(&self) {
        unimplemented!()
    }

    /// All internal objects one step lower in hierarchy.
    pub fn internal_sub_objects(&self) {
        unimplemented!()
    }

    /// All private objects one step lower in hierarchy.
    pub fn private_sub_objects(&self) {
        unimplemented!()
    }

    /// All public servies at current level of hierarchy.
    pub fn public_services(&self) {
        unimplemented!()
    }

    /// All internal services at current level of hierarchy.
    pub fn internal_services(&self) {
        unimplemented!()
    }

    /// All private servies at current level of hierarchy.
    pub fn private_services(&self) {
        unimplemented!()
    }
}

impl InterfaceHierarchy {

    /// Current package.
    pub fn current_package(&self) {
        unimplemented!()
    }

    /// All interfaces located at current level of the package.
    pub fn interfaces(&self) {
        unimplemented!()
    }

    /// All packages at one step lower level of hierarchy.
    pub fn sub_packages(&self) {
        unimplemented!()
    }
}
