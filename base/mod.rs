/// Network object. An implementation of some interfaces. It has it's own
/// name. It is located in some other network and has internal network in
/// itself. The object has it's unique ID which allows to distinguish objects
/// with same names and locations.
pub struct Object {
}

/// Interface with given set of services. Gives information about interface
/// vendor, version, services and sub-interfaces in it. Also gives information
/// about required interface implementations in order to allow object implement
/// this interface.
pub struct Interface {
}

/// Detailed information about service. It's parent interface, name, version
/// etc.
pub struct Service {
}

/// Service implementation in the exact object. Contains all the information
/// required to run the service.
pub struct ServiceImpl {
}

/// Registry with all network elements in it. Registry is a root network object
/// that hold all other objects in the network environment.
struct Registry {
}
