use std::collections::HashMap;

/// Network object. An implementation of some interfaces. It has it's own
/// name. It is located in some other network and has internal network in
/// itself. The object has it's unique ID which allows to distinguish objects
/// with same names and locations.
pub struct Object<'a> {

    /// Object unique identifier in it's network level.
    id          : u32,

    /// Object name.
    name        : String,

    /// Parent object if any. If this is the highest object in the hierarchy
    /// the field will have None value.
    parent      : Option<&'a Object<'a>>,

    /// Services implemented by the object at current network level.
    services    : ServiceReg<'a>,

    /// Internal object network registry.
    registry    : Registry<'a>,
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
pub struct ServiceImpl<'a> {

    /// Basic information about the service.
    base        : &'a Service,

    addr        : ServiceAddr,
}

/// Service registry.
#[derive(Default)]
struct ServiceReg<'a> {

    pub private     : ServiceHashMap<'a>,

    pub internal    : ServiceHashMap<'a>,

    pub public      : ServiceHashMap<'a>,
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

#[derive(Default)]
struct ObjectHashMap<'a> {

    pub map     : HashMap<&'a str, Object<'a>>,
}

#[derive(Default)]
struct ServiceHashMap<'a> {

    pub map     : HashMap<&'a str, ServiceImpl<'a>>,
}

/// Registry with all network objects in it. Registry is a root network object
/// that hold all other objects in the network environment.
#[derive(Default)]
struct Registry<'a> {

    pub pub_obj     : ObjectHashMap<'a>,

    pub int_obj     : ObjectHashMap<'a>,

    pub priv_obj    : ObjectHashMap<'a>,
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

impl<'a> Object<'a> {

    pub fn new(id: u32, name: String, parent: Option<&'a Object>)
            -> Object<'a> {
        Object {
            id,
            name,
            parent,
            services: Default::default(),
            registry: Default::default(),
        }
    }
}
