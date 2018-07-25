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

/// Transaction allows making multiple changes to object as a single
/// operation. This allows to revert changes if one of the changes
/// fails.
pub struct ObjectTransaction {
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

    pub fn new() {
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

    pub fn remove_public_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_internal_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_private_service(&mut self) {
        unimplemented!()
    }

    /// Mark this object as interface implementer.
    pub fn implement_interface(&mut self) {
        unimplemented!()
    }
}
