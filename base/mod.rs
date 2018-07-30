use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Package path module.
mod path;

pub use self::path::Path            as PackagePath;
pub use self::path::PathConstructor as PackagePathConstructor;
pub use self::path::PathIter        as PackagePathIter;
pub use self::path::PathNode        as PackagePathNode;

/// The object of the network. Object contains services and subobjects.
/// It can also implement some interfaces. It has some internal memory
/// shared among internal objects and services.
pub struct Object {
}

/// Transaction allows making multiple changes to object as a single
/// operation. This allows to revert changes if one of the changes
/// fails.
pub struct ObjectTransaction {
}

/// Interface forms a set of services with defined functionality. When this
/// functionality gets needed, master reads the interface information
/// and finds appropriate object that implements this interface and thus can
/// solve some task with implemented interface functions.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Interface {

    name    : String,
    dep     : InterfaceDependency,
}

/// Package contains set of interfaces that solve similar tasks or have
/// same vendor.
pub struct Package {

    path    : PackagePath,
}

/// Service is called when some object needs to solve some problem which
/// this service can carry out.
pub struct Service {

    name    : String,
}

/// Dependency on iterface implementation. Shows what interfaces should
/// be implemented in order to allow some other interface to be
/// implemented by same object.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InterfaceDependency {

    tree    : BTreeSet<Rc<Interface>>,
}

/// Version of the interface. Contains major, minor and patch parts.
#[derive(PartialEq, Eq)]
pub struct InterfaceVersion {

    major   : u32,
    minor   : u32,
    patch   : u32,
}

/// Architecture-dependent fields and functions of service.
pub trait ServiceArch {

    /// Service component of ServiceArch trait.
    fn service(&self) -> &Service;
}

impl Object {

    /// All public services at current network level of the object.
    pub fn public_services(&self) {
        unimplemented!()
    }

    /// All internal services at current network level of the object.
    pub fn internal_services(&self) {
        unimplemented!()
    }

    /// All private services at current network level of the object.
    pub fn private_services(&self) {
        unimplemented!()
    }

    /// All interfaces implemented by this object.
    pub fn interfaces(&self) {
        unimplemented!()
    }

    pub fn apply_transaction(&mut self) {
        unimplemented!()
    }
}

impl ObjectTransaction {

    pub fn new() -> Self {
        unimplemented!()
    }

    /// Add new public service.
    pub fn add_public_service(&mut self) {
        unimplemented!()
    }

    /// Add new internal service.
    pub fn add_internal_service(&mut self) {
        unimplemented!()
    }

    /// Add new private service.
    pub fn add_private_service(&mut self) {
        unimplemented!()
    }

    /// Add service and set it's visibility to given value.
    pub fn add_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_public_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_internal_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_private_service(&mut self) {
        unimplemented!()
    }

    /// Change visibility of given service. If service initial visibility
    /// isn't passed then all lists will be checked to find requested service
    /// and then the visibility will be changed.
    pub fn change_service_access(&mut self) {
        unimplemented!()
    }

    /// Create new public sub-object.
    pub fn new_public_sub_object(&mut self) {
        unimplemented!()
    }

    /// Create new internal sub-object.
    pub fn new_internal_sub_object(&mut self) {
        unimplemented!()
    }

    /// Create new private sub-object.
    pub fn new_private_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete private sub-object.
    pub fn remove_private_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete internal sub-object.
    pub fn remove_internal_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete public sub-object.
    pub fn remove_public_sub_object(&mut self) {
        unimplemented!()
    }

    /// Change the visibility of sub-object from given value to new value.
    /// If initial value is not passed then every list of objects will be
    /// checked to find it and change the visibility to appropriate.
    pub fn change_sub_object_access(&mut self) {
        unimplemented!()
    }

    /// Mark this object as interface implementer.
    pub fn implement_interface(&mut self) {
        unimplemented!()
    }

    /// Tell this object now doesn't implement given interface.
    pub fn unimplement_interface(&mut self) {
        unimplemented!()
    }
}

impl Interface {

    /// Package where this interface is located.
    pub fn package(&self) {
        unimplemented!()
    }

    /// The name of the interface as it appear in the package.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The version of this interface.
    pub fn version(&self) {
        unimplemented!()
    }

    /// List with all services that must be implemented by this interface
    /// implementer. This exludes the services of dependent interfaces.
    pub fn services(&self) {
        unimplemented!()
    }

    /// List of dependent interfaces that must be implemented in order to
    /// allow implementing this interface.
    pub fn dependency(&self) -> &InterfaceDependency {
        &self.dep
    }
}

impl Package {

    /// The absolute path to this package.
    pub fn path(&self) -> &PackagePath {
        &self.path
    }
}

impl Service {

    /// The name of this service.
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl InterfaceDependency {

    /// Create new InterfaceDependency with empty dependency list.
    pub fn new() -> Self {
        InterfaceDependency { tree: BTreeSet::new() }
    }

    /// Tree of interfaces that are in this dependency.
    pub fn tree(&self) -> &BTreeSet<Rc<Interface>> {
        &self.tree
    }

    /// Tree of interfaces that are in this dependency.
    pub fn tree_mut(&mut self) -> &mut BTreeSet<Rc<Interface>> {
        &mut self.tree
    }

    /// Add new interface to the dependency.
    pub fn add(&mut self, interface_rc: &Rc<Interface>) {
        self.tree.insert(interface_rc.clone());
    }

    /// True if given interface is in this dependency.
    pub fn includes(&self, i: &Interface) -> bool {
        self.tree.contains(i)
    }
}

impl Ord for InterfaceVersion {

    fn cmp(&self, other: &InterfaceVersion) -> Ordering {
        use self::Ordering::*;

        if self.major > other.major {
            Greater
        } else if self.major < other.major {
            Less
        } else if self.minor > other.minor {
            Greater
        } else if self.minor < other.minor {
            Less
        } else if self.patch > other.patch {
            Greater
        } else if self.patch < other.patch {
            Less
        } else {
            Equal
        }
    }
}

impl PartialOrd for InterfaceVersion {

    fn partial_cmp(&self, other: &InterfaceVersion) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl InterfaceVersion {

    /// Mix given version components into single InterfaceVersion struct.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        InterfaceVersion { major, minor, patch }
    }

    /// Major version of the interface.
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Minor version of the interface.
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Patch number of the interface version.
    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// Check whether implementer of this interface version can be used
    /// by object with given interface version.
    pub fn is_compatible(&self, other: &InterfaceVersion) -> bool {
        if self.major != other.major {
            false
        } else if self.minor < other.minor {
            false
        } else {
            true
        }
    }
}
