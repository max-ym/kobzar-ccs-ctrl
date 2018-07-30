use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Package path module.
mod path;

/// Package implementing ObjectTransation-s.
mod objtrans;

pub use self::path::Path            as PackagePath;
pub use self::path::PathConstructor as PackagePathConstructor;
pub use self::path::PathIter        as PackagePathIter;
pub use self::path::PathNode        as PackagePathNode;

pub use self::objtrans::*;

/// The object of the network. Object contains services and subobjects.
/// It can also implement some interfaces. It has some internal memory
/// shared among internal objects and services.
pub struct Object {
}

/// Interface forms a set of services with defined functionality. When this
/// functionality gets needed, master reads the interface information
/// and finds appropriate object that implements this interface and thus can
/// solve some task with implemented interface functions.
#[derive(PartialEq, Eq)]
pub struct Interface {

    name    : String,
    dep     : InterfaceDependency,
    ver     : InterfaceVersion,
    pack    : Rc<Package>,
    serv    : BTreeSet<Service>,
}

/// Package contains set of interfaces that solve similar tasks or have
/// same vendor.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Package {

    path    : PackagePath,
}

/// Service is called when some object needs to solve some problem which
/// this service can carry out.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(PartialEq, Eq, Clone, Copy)]
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

impl Ord for Interface {

    fn cmp(&self, other: &Interface) -> Ordering {
        use self::Ordering::*;

        let cmp_name = self.name.cmp(&other.name);
        match cmp_name {
            Greater => Greater,
            Less    => Less,
            Equal   => {
                let cmp_version = self.ver.cmp(&other.ver);
                match cmp_version {
                    Greater => Greater,
                    Less    => Less,
                    Equal   => {
                        let cmp_pack = self.pack.cmp(&other.pack);
                        match cmp_pack {
                            Greater => Greater,
                            Less    => Less,
                            Equal   => {
                                let cmp_serv = self.serv.cmp(&other.serv);
                                match cmp_serv {
                                    Greater => Greater,
                                    Less    => Less,
                                    Equal   => {
                                        let cmp_dep = self.dep.cmp(&other.dep);
                                        match cmp_dep {
                                            Greater => Greater,
                                            Less    => Less,
                                            Equal   => Equal
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Interface {

    fn partial_cmp(&self, other: &Interface) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Interface {

    /// Package where this interface is located.
    pub fn package(&self) -> &Rc<Package> {
        &self.pack
    }

    /// The name of the interface as it appear in the package.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The version of this interface.
    pub fn version(&self) -> &InterfaceVersion {
        &self.ver
    }

    /// List with all services that must be implemented by this interface
    /// implementer. This exludes the services of dependent interfaces.
    pub fn services(&self) -> &BTreeSet<Service> {
        &self.serv
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
