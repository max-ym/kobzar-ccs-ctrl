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
    pub fn unimplemente_interface(&mut self) {
        unimplemented!()
    }
}
