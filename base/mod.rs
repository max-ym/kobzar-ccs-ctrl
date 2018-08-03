use std::rc::{Rc, Weak};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::btree_map::Entry as BTreeMapEntry;

/// Package path module.
mod path;

pub use self::path::Path;
pub use self::path::PathConstructor;
pub use self::path::PathIter;
pub use self::path::PathNode;

pub type ServiceMapEntry<'a> = BTreeMapEntry<
        'a, NameWrap, Box<ServiceArch>>;

/// Wrap to allow creating pointer to name string of Service or Object
/// and use it as
/// readable reference. Used to avoid cloning strings in BTrees used to
/// search services.
struct NameWrap(*const String);

/// The object of the network. Object contains services and subobjects.
/// It can also implement some interfaces. It has some internal memory
/// shared among internal objects and services.
pub struct Object {

    /// Name of this object.
    name        : String,

    /// Public service set.
    pubsrv      : BTreeSet<Box<ServiceArch>>,

    /// Internal service set.
    intsrv      : BTreeSet<Box<ServiceArch>>,

    /// Private service set.
    privsrv     : BTreeSet<Box<ServiceArch>>,

    /// Interfaces implemented.
    ints        : BTreeSet<Interface>,

    /// Service names tree. Allows to quickly find whether the service
    /// with given name already exist and access it.
    srvnames    : BTreeMap<NameWrap, ServiceMapEntry<'static>>,
}

/// Object and it's ID in the network. Allows to distinguish objects with
/// same names on same hierarchy level.
pub struct ObjectVector {
    obj : Object,
    id  : u32,
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

    path    : Path,
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

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// All public services at current network level of the object.
    pub fn public_services(&self) -> &BTreeSet<Box<ServiceArch>> {
        &self.pubsrv
    }

    /// All internal services at current network level of the object.
    pub fn internal_services(&self) -> &BTreeSet<Box<ServiceArch>> {
        &self.intsrv
    }

    /// All private services at current network level of the object.
    pub fn private_services(&self) -> &BTreeSet<Box<ServiceArch>> {
        &self.privsrv
    }

    /// All interfaces implemented by this object.
    pub fn interfaces(&self) -> &BTreeSet<Interface> {
        &self.ints
    }

    /// Whether this object has service with this name.
    pub fn has_service_with_name(&self, name: &String) -> bool {
        let val = self.srvnames.get(name);
        val.is_some()
    }

    /// Service entry in BTree by given service name if it is implemented
    /// in this object.
    pub fn service_by_name(&self, name: &String)
            -> Option<&ServiceMapEntry> {
        let val = self.srvnames.get(name);

        if val.is_none() {
            return None;
        }
        let val = val.unwrap();

        Some(val)
    }

    /// Service entry in BTree by given service name if it is implemented
    /// in this object.
    pub fn service_by_name_mut(&mut self, name: &String) ->
            Option<&mut ServiceMapEntry> {
        // Convert const reference to mut from related fn.
        match self.service_by_name(name) {
            Some(v) => Some(unsafe { &mut *(v as *const _ as *mut _) }),
            None    => None
        }
    }
}

impl PartialEq for ObjectVector {

    fn eq(&self, other: &ObjectVector) -> bool {
        self.id == other.id
    }
}

impl Eq for ObjectVector {}

impl PartialOrd for ObjectVector {

    fn partial_cmp(&self, other: &ObjectVector) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ::std::borrow::Borrow<Object> for ObjectVector {

    fn borrow(&self) -> &Object {
        &self.obj
    }
}

impl ::std::borrow::BorrowMut<Object> for ObjectVector {

    fn borrow_mut(&mut self) -> &mut Object {
        &mut self.obj
    }
}

impl ObjectVector {

    pub fn new(obj: Object, id: u32) -> Self {
        ObjectVector { obj, id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn split_vector(self) -> (Object, u32) {
        (self.obj, self.id)
    }
}

impl Ord for ObjectVector {

    fn cmp(&self, other: &ObjectVector) -> Ordering {
        self.id.cmp(&other.id)
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
    pub fn path(&self) -> &Path {
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

impl PartialEq for NameWrap {

    fn eq(&self, other: &NameWrap) -> bool {
        unsafe { *self.0 == *other.0 }
    }
}

impl Eq for NameWrap {}

impl Ord for NameWrap {

    fn cmp(&self, other: &NameWrap) -> Ordering {
        let reference = unsafe { &*self.0 };
        let otherref  = unsafe { &*other.0 };

        reference.cmp(otherref)
    }
}

impl PartialOrd for NameWrap {

    fn partial_cmp(&self, other: &NameWrap) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ::std::borrow::Borrow<String> for NameWrap {

    fn borrow(&self) -> &String {
        unsafe { &*self.0 }
    }
}
