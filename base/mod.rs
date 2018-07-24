use std::collections::HashMap;
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

/// Interface with given set of services. Gives information about interface
/// vendor, version, services and sub-interfaces in it. Also gives information
/// about required interface implementations in order to allow object implement
/// this interface.
pub struct Interface {

    /// Vendor of this interface.
    vendor      : String,

    /// Name of this interface.
    name        : String,

    /// Version of the interface.
    version     : InterfaceVersion,

    /// Services contained by the interface. All listed services must be
    /// public.
    services    : Vec<Service>,

    /// Subinterfaces required to be located inside of this interface.
    /// Interfaces must be public.
    subints     : Vec<Interface>,

    /// All interfaces that must be implemented by the object that wishes to
    /// implement this interface.
    require     : Vec<Interface>,
}

/// Detailed information about service. It's parent interface, name, version
/// etc.
pub struct Service {

    /// Service name.
    name        : String,

    version     : ServiceVersion,
}

/// Visibility of network element.
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
struct ServiceReg {

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
pub struct InterfaceVersion {

    major   : u32,

    minor   : u32,
}

/// Implemented service interface version. Different major versions are
/// not compatible. Lower or equal required minor version allows to make a
/// call for the service from the calling object.
pub struct ServiceVersion {

    major   : u32,

    minor   : u32,
}

/// Hash created to map the object by it's name.
#[derive(PartialEq, Eq, Hash)]
struct ObjectHash {
    val     : u64,
}

/// Hash created to map the service by it's name.
#[derive(PartialEq, Eq, Hash)]
struct ServiceHash {
    val     : u64,
}

#[derive(Default)]
struct ObjectHashMap {

    pub map     : HashMap<ObjectHash, Object>,
}

#[derive(Default)]
struct ServiceHashMap {

    pub map     : HashMap<ServiceHash, ServiceImpl>,
}

/// Registry with all network objects in it. Registry is a root network object
/// that hold all other objects in the network environment.
#[derive(Default)]
struct Registry {

    pub pub_obj     : ObjectHashMap,

    pub int_obj     : ObjectHashMap,

    pub priv_obj    : ObjectHashMap,
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
}
