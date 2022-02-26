use crate::StandardOptions;
use zbus::dbus_proxy;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct LoopSetupOptions {
    #[zvariant(rename = "auth.no_user_interaction")]
    pub no_user_interaction: bool,
    #[zvariant(rename = "offset")]
    pub offset: u64,
    #[zvariant(rename = "size")]
    pub size: u64,
    #[zvariant(rename = "read-only")]
    pub readonly: bool,
    #[zvariant(rename = "no-part-scan")]
    pub no_part_scan: bool,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct DeviceSpec {
    #[zvariant(rename = "path")]
    pub path: Option<String>,
    #[zvariant(rename = "label")]
    pub label: Option<String>,
    #[zvariant(rename = "uuid")]
    pub uuid: Option<String>,
}

#[dbus_proxy(
    interface = "org.freedesktop.UDisks2.Manager",
    default_service = "org.freedesktop.UDisks2",
    default_path = "/org/freedesktop/UDisks2/Manager"
)]
pub trait UDisks2Manager {
    /// Tests for availability to create the given filesystem. See the
    /// [`Self::supported_filesystems`] property for a list of known types. Unknown or unsupported
    /// filesystems result in an error.
    fn can_format(&self, type_: &str) -> zbus::Result<(bool, String)>;

    /// Tests for availability to resize the given filesystem. The mode flags indicate if growing
    /// and/or shriking resize is available if mounted/unmounted. The mode corresponds to bitwise-OR
    /// combined BDFsResizeFlags of the libblockdev FS plugin: BD_FS_OFFLINE_SHRINK = 2, shrinking
    /// resize allowed when unmounted BD_FS_OFFLINE_GROW = 4, growing resize allowed when unmounted
    /// BD_FS_ONLINE_SHRINK = 8, shrinking resize allowed when mounted BD_FS_ONLINE_GROW = 16,
    /// growing resize allowed when mounted Unknown filesystems or filesystems which do not support
    /// resizing result in errors.
    fn can_resize(&self, type_: &str) -> zbus::Result<(bool, u64, String)>;

    /// Tests for availability to check the given filesystem. Unsupported filesystems or filesystems
    /// which do not support checking result in errors.
    fn can_check(&self, type_: &str) -> zbus::Result<(bool, String)>;

    /// Tests for availability repair the given filesystem. Unsupported filesystems or filesystems
    /// which do not support repairing result in errors.
    fn can_repair(&self, type_: &str) -> zbus::Result<(bool, String)>;

    /// Loads and activates single module identified by name. In case the module is already active
    /// no reinitialization is done and this call has no effect. Clients are supposed to call this
    /// method before using any of the particular module API. In case no such module is available or
    /// its initialization fails a proper error is returned. This action also causes all objects to
    /// receive add uevent that allows the module to attach extra interfaces before this method call
    /// returns.
    ///
    /// Modules cannot be deactivated at the moment.
    fn enable_module(&self, name: &str, enable: bool) -> zbus::Result<()>;

    /// Loads and activates modules. Modules that have been already loaded are not reinitialized on
    /// subsequent calls to this method and are skipped. In case any new module is getting activated
    /// by this method call a add uevent is triggered on all exported objects.
    ///
    /// This takes in account an optional explicit list of modules to load as specified in the
    /// `/etc/udisks2/udisks2.conf` config file. If unspecified all available modules will be
    /// loaded.
    ///
    /// Modules cannot be deactivated at the moment. This method call never fails even if no module
    /// has been activated and by nature it cannot report any particular module initialization
    /// failures. Clients have no way of finding that a particular module is available.
    #[deprecated]
    fn enable_modules(&self, enable: bool) -> zbus::Result<()>;

    /// Get list of all block devices known to UDisks.
    fn get_block_devices(
        &self,
        options: StandardOptions,
    ) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Get device(s) matching specification given in devspec.
    fn resolve_device(
        &self,
        devspec: DeviceSpec,
        options: StandardOptions,
    ) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Creates a block device for the file represented by fd.
    fn loop_setup(
        &self,
        fd: zbus::zvariant::Fd,
        options: LoopSetupOptions,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Creates a new RAID array on the block devices specified by blocks. Each element in this
    /// array must be an object path to an object implementing the org.freedesktop.UDisks2.Block interface.
    ///
    /// Known and supported values for level include “raid0”, “raid1”, “raid4”, “raid5”, “raid6” and
    /// “raid10”.
    ///
    /// Before the array is created, all devices in blocks are erased. Once created (but before the
    /// method returns), the RAID array will be erased.
    fn mdraid_create(
        &self,
        blocks: &[zbus::zvariant::ObjectPath<'_>],
        level: &str,
        name: &str,
        chunk: u64,
        options: StandardOptions,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// The default encryption type.
    #[dbus_proxy(property)]
    fn default_encryption_type(&self) -> zbus::Result<String>;

    /// List of supported encryption types by UDisks2.
    #[dbus_proxy(property)]
    fn supported_encryption_types(&self) -> zbus::Result<Vec<String>>;

    /// List of supported filesystem by UDisks2.
    #[dbus_proxy(property)]
    fn supported_filesystems(&self) -> zbus::Result<Vec<String>>;

    /// The version of the daemon currently running.
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;
}
