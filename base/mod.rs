/// The object of the network. Object contains services and subobjects.
/// It can also implement some interfaces. It has some internal memory
/// shared among internal objects and services.
pub struct Object {
}

/// Interface forms a set of services with defined functionality. When this
/// functionality gets needed, master reads the interface information
/// and finds appropriate object that implements this interface and thus can
/// solve some task with implemented interface functions.
pub struct Interface {
}

/// Package contains set of interfaces that solve similar tasks or have
/// same vendor.
pub struct Package {
}

/// Service is called when some object needs to solve some problem which
/// this service can carry out.
pub struct Service {
}
