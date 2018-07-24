use std::collections::HashMap;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Network object. An implementation of some interfaces. It has it's own
/// name. It is located in some other network and has internal network in
/// itself. The object has it's unique ID which allows to distinguish objects
/// with same names and locations.
pub struct Object {

    /// Object unique identifier in it's network level.
    id          : u32,

    /// Object name.
    name        : String,

    /// Parent object if any. If this is the highest object in the hierarchy
    /// the field will have None value.
    parent      : Option<Rc<Object>>,

    /// Services implemented by the object at current network level.
    services    : ServiceReg,

    /// Internal object network registry.
    registry    : Registry,
}

/// Vendor name and full path to the interface package.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct InterfacePath {

    /// Name of the package at current level.
    name        : String,

    /// Higher level of this path hierarchy. If current node is at top this
    /// field is None.
    parent      : Option<Rc<InterfacePath>>,
}

/// Interface with given set of services. Gives information about interface
/// vendor, version, services and sub-interfaces in it. Also gives information
/// about required interface implementations in order to allow object implement
/// this interface.
pub struct Interface {

    /// Vendor of this interface.
    vendor      : InterfacePath,

    /// Name of this interface.
    name        : String,

    /// Version of the interface.
    version     : InterfaceVersion,

    /// Services contained by the interface. All listed services must be
    /// public.
    services    : Vec<Rc<Service>>,

    /// Subinterfaces required to be located inside of this interface.
    /// Interfaces must be public.
    subints     : Vec<Rc<Interface>>,

    /// All interfaces that must be implemented by the object that wishes to
    /// implement this interface.
    require     : Vec<Rc<Interface>>,
}

/// Set of interface implementers.
pub struct InterfaceImplementers {

    /// Interface being implemented.
    interface   : Rc<Interface>,

    /// Objects that implement this interface.
    objects     : LinkedList<Rc<Object>>,
}

/// Detailed information about service. It's parent interface, name, version
/// etc.
pub struct Service {

    /// Service name.
    name        : String,

    version     : ServiceVersion,
}

/// Visibility of network element.
#[derive(Clone, Copy)]
pub enum Visibility {

    /// Object is visible from the upper level network and from lower level
    /// network for all objects that can access that network.
    Public,

    /// Visible only for current network level and all children of this
    /// network.
    Internal,

    /// Visible only for current network level, child and parent networks
    /// cannot access the element.
    Private,
}

/// Service implementation in the exact object. Contains all the information
/// required to run the service.
pub struct ServiceImpl {

    /// Basic information about the service.
    base        : Rc<Service>,

    addr        : ServiceAddr,
}

/// Service registry.
#[derive(Default)]
pub struct ServiceReg {

    pub private     : ServiceHashMap,

    pub internal    : ServiceHashMap,

    pub public      : ServiceHashMap,
}

/// Address of service executable code.
#[derive(Clone, Copy)]
pub struct ServiceAddr {

    addr        : usize,
}

/// Interface version. Different major number means incompatible changes to
/// the interface. Objects that require specific major version must seek
/// for implementer with given major version. Object can connect to the
/// implementer with equal or greater minor number.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct InterfaceVersion {

    major   : u32,

    minor   : u32,
}

/// Implemented service interface version. Different major versions are
/// not compatible. Lower or equal required minor version allows to make a
/// call for the service from the calling object.
#[derive(Clone, Copy)]
pub struct ServiceVersion {

    major   : u32,

    minor   : u32,
}

/// Hash created to map the object by it's name.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ObjectHash {
    val     : u64,
}

/// Hash created to map the service by it's name.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ServiceHash {
    val     : u64,
}

#[derive(Default)]
pub struct ObjectHashMap {

    pub map     : HashMap<ObjectHash, Object>,
}

#[derive(Default)]
pub struct ServiceHashMap {

    pub map     : HashMap<ServiceHash, ServiceImpl>,
}

#[derive(Default)]
pub struct ImplementersVec {

    pub im  : Vec<InterfaceImplementers>,
}

/// Registry with all network objects in it. Registry is a root network object
/// that hold all other objects in the network environment.
#[derive(Default)]
struct Registry {

    /// Public objects.
    pub pub_obj     : ObjectHashMap,

    /// Internal objects.
    pub int_obj     : ObjectHashMap,

    /// Private objects.
    pub priv_obj    : ObjectHashMap,

    /// ID to be assigned to newly created sub-objects.
    pub new_obj_id  : u32,

    /// Public interface implementers.
    pub pub_obj_int : ImplementersVec,

    /// Internal interface implementers.
    pub int_obj_int : ImplementersVec,

    /// Private interface implementers.
    pub priv_obj_int: ImplementersVec,
}

impl ServiceVersion {

    pub fn new(major: u32, minor: u32) -> Self {
        ServiceVersion { major, minor }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

impl InterfaceVersion {

    pub fn new(major: u32, minor: u32) -> Self {
        InterfaceVersion { major, minor }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

impl From<usize> for ServiceAddr {

    fn from(addr: usize) -> Self {
        ServiceAddr { addr }
    }
}

impl Into<usize> for ServiceAddr {

    fn into(self) -> usize {
        self.addr
    }
}

impl ObjectHash {

    /// Generate object name hash from given string.
    pub fn from_str(s: &String) -> Self {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        ObjectHash { val : hasher.finish() }
    }

    /// Generate object name hash for given object.
    pub fn from_obj(o: &Object) -> Self {
        Self::from_str(&o.name)
    }
}

impl ServiceHash {

    /// Generate service name hash from given string.
    pub fn from_str(s: &String) -> Self {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        ServiceHash { val : hasher.finish() }
    }

    /// Generate service name hash for given object.
    pub fn from_srv(s: &Service) -> Self {
        Self::from_str(&s.name)
    }

    pub fn from_srv_impl(s: &ServiceImpl) -> Self {
        Self::from_srv(&s.base)
    }
}

impl Object {

    pub fn new(id: u32, name: String, parent: Option<Rc<Object>>)
            -> Object {
        Object {
            id,
            name,
            parent,
            services: Default::default(),
            registry: Default::default(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn parent(&self) -> Option<Rc<Object>> {
        self.parent.clone()
    }

    pub fn services(&self) -> &ServiceReg {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut ServiceReg {
        &mut self.services
    }
}

impl Interface {

    pub fn new(vendor: InterfacePath, name: String, version: InterfaceVersion)
            -> Self {
        Interface {
            vendor,
            name,
            version,
            services: Default::default(),
            subints: Default::default(),
            require: Default::default(),
        }
    }

    pub fn vendor(&self) -> &InterfacePath {
        &self.vendor
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &InterfaceVersion {
        &self.version
    }

    pub fn services(&self) -> &Vec<Rc<Service>> {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut Vec<Rc<Service>> {
        &mut self.services
    }

    pub fn subinterfaces(&self) -> &Vec<Rc<Interface>> {
        &self.subints
    }

    pub fn subinterfaces_mut(&mut self) -> &mut Vec<Rc<Interface>> {
        &mut self.subints
    }

    pub fn required_interfaces(&self) -> &Vec<Rc<Interface>> {
        &self.require
    }

    pub fn required_interfaces_mut(&mut self) -> &mut Vec<Rc<Interface>> {
        &mut self.require
    }
}

impl Hash for Interface {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vendor.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Interface {

    fn eq(&self, other: &Interface) -> bool {
        if self.vendor != other.vendor {
            false
        } else {
            self.name == other.name && self.version == other.version
        }
    }
}

impl Eq for Interface {}
