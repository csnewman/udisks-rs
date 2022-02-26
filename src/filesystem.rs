use crate::ascii::AsciiString;
use crate::StandardOptions;
use zbus::dbus_proxy;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct MountOptions {
    #[zvariant(rename = "auth.no_user_interaction")]
    pub no_user_interaction: bool,
    #[zvariant(rename = "fstype")]
    pub fs_type: String,
    #[zvariant(rename = "options")]
    pub mount_options: String,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct UnMountOptions {
    #[zvariant(rename = "auth.no_user_interaction")]
    pub no_user_interaction: bool,
    #[zvariant(rename = "force")]
    pub force: bool,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct TakeOwnershipOptions {
    #[zvariant(rename = "auth.no_user_interaction")]
    pub no_user_interaction: bool,
    #[zvariant(rename = "recursive")]
    pub recursive: bool,
}

#[dbus_proxy(
    interface = "org.freedesktop.UDisks2.Filesystem",
    default_service = "org.freedesktop.UDisks2"
)]
trait Filesystem {
    /// Sets the filesystem label.
    fn set_label(&self, label: &str, options: StandardOptions) -> zbus::Result<()>;

    /// Mounts the filesystem.
    ///
    /// The directory the filesystem will be mounted in is determined by looking at data related to
    /// the device or filesystem (such the filesystem UUID and label) and will be created
    /// automatically except if the device the filesystem resides on is referenced in the
    /// /etc/fstab file, see below. In either case, the directory the filesystem is mounted in, is
    /// returned in `mount_path` on success - it is usually a sub-directory of
    /// `/run/media/$USER` but note that any directory may be returned.
    ///
    /// The filesystem type to use can be overridden with the fstype option and mount options
    /// (a comma-separated string) can be given in options option. Note that both the mount options
    /// and filesystem types are validated against a (small) whitelist to avoid unexpected privilege
    /// escalation
    ///
    /// If the device in question is referenced in the `/etc/fstab` file, the mount command is
    /// called directly (as root) and the given options or filesystem type given in options are
    /// ignored.
    ///
    /// If x-udisks-auth is specified as an option for the device in the /etc/fstab file, then the
    /// mount command is run as the calling user, without performing any authorization check
    /// mentioned above. If this fails because of insufficient permissions, an authorization check
    /// is performed (which typically results in the user having to authenticate as an
    /// administrator). If authorized, the mount command is then run as root.
    ///
    /// The filesystem should be unmounted using the Unmount() method.
    ///
    /// If the device is removed without being unmounted (e.g. the user yanking the device or
    /// pulling the media out) or unmounted in a way that bypasses the Unmount() method (e.g.
    /// unmounted by the super-user by using the umount(8) command directly), the device will be
    /// unmounted (if needed) and/or the mount point will be cleaned up.
    fn mount(&self, options: MountOptions) -> zbus::Result<String>;

    /// Unmount a mounted device.
    ///
    /// If the device in question was mounted by the calling user via the [`Self::Mount()`] method
    /// the filesystem is unmounted without any authorization checks. Otherwise, an authorization
    /// check is performed (which typically results in the user having to authenticate as an
    /// administrator). If authorized, the filesystem is unmounted.
    ///
    /// If the mountpoint was previously created by udisks it is guaranteed it will be removed upon
    /// returning from this method call.
    ///
    /// If the filesystem is busy, this operation fails with the error
    /// `org.freedesktop.UDisks2.Error.DeviceBusy` unless the force option is used.
    fn unmount(&self, options: UnMountOptions) -> zbus::Result<()>;

    /// Resizes the filesystem.
    ///
    /// Shrinking operations need to move data which causes this action to be slow. The
    /// filesystem-resize job for the object might expose progress.
    fn resize(&self, size: u64, options: StandardOptions) -> zbus::Result<()>;

    /// Checks the filesystem for consistency.
    ///
    /// Unsupported filesystems result in an error.
    fn check(&self, options: StandardOptions) -> zbus::Result<bool>;

    /// Tries to repair the filesystem.
    ///
    /// Unsupported filesystems result in an error.
    fn repair(&self, options: StandardOptions) -> zbus::Result<bool>;

    /// Changes ownership of the filesystem to the UID and GID of the calling user.
    ///
    /// Filesystems that don't support ownership result in an error.
    fn take_ownership(&self, options: TakeOwnershipOptions) -> zbus::Result<()>;

    /// An array of filesystems paths for where the file system on the device is mounted. If the
    /// device is not mounted, this array is empty.
    #[dbus_proxy(property)]
    fn mount_points(&self) -> zbus::Result<Vec<AsciiString>>;

    /// The size of the filesystem. This is the amount of bytes used on the block device. If this is
    /// smaller than `org.freedesktop.Udisks2.Block.Size`, then the filesystem can be made larger
    /// with Resize.
    ///
    /// If the size is unknown, the property is zero.
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;
}
