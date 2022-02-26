use zbus::dbus_proxy;

use crate::ascii::AsciiString;

#[dbus_proxy(
    interface = "org.freedesktop.UDisks2.Block",
    default_service = "org.freedesktop.UDisks2"
)]
trait Block {
    /// AddConfigurationItem method
    fn add_configuration_item(
        &self,
        item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Format method
    fn format(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// GetSecretConfiguration method
    fn get_secret_configuration(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// OpenDevice method
    fn open_device(
        &self,
        mode: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// OpenForBackup method
    fn open_for_backup(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// OpenForBenchmark method
    fn open_for_benchmark(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// OpenForRestore method
    fn open_for_restore(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedFd>;

    /// RemoveConfigurationItem method
    fn remove_configuration_item(
        &self,
        item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Rescan method
    fn rescan(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UpdateConfigurationItem method
    fn update_configuration_item(
        &self,
        old_item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        new_item: &(
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Configuration property
    #[dbus_proxy(property)]
    fn configuration(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// CryptoBackingDevice property
    #[dbus_proxy(property)]
    fn crypto_backing_device(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Device property
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::Result<AsciiString>;

    /// DeviceNumber property
    #[dbus_proxy(property)]
    fn device_number(&self) -> zbus::Result<u64>;

    /// Drive property
    #[dbus_proxy(property)]
    fn drive(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// HintAuto property
    #[dbus_proxy(property)]
    fn hint_auto(&self) -> zbus::Result<bool>;

    /// HintIconName property
    #[dbus_proxy(property)]
    fn hint_icon_name(&self) -> zbus::Result<String>;

    /// HintIgnore property
    #[dbus_proxy(property)]
    fn hint_ignore(&self) -> zbus::Result<bool>;

    /// HintName property
    #[dbus_proxy(property)]
    fn hint_name(&self) -> zbus::Result<String>;

    /// HintPartitionable property
    #[dbus_proxy(property)]
    fn hint_partitionable(&self) -> zbus::Result<bool>;

    /// HintSymbolicIconName property
    #[dbus_proxy(property)]
    fn hint_symbolic_icon_name(&self) -> zbus::Result<String>;

    /// HintSystem property
    #[dbus_proxy(property)]
    fn hint_system(&self) -> zbus::Result<bool>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// IdLabel property
    #[dbus_proxy(property)]
    fn id_label(&self) -> zbus::Result<String>;

    /// IdType property
    #[dbus_proxy(property)]
    fn id_type(&self) -> zbus::Result<String>;

    /// IdUUID property
    #[dbus_proxy(property)]
    fn id_uuid(&self) -> zbus::Result<String>;

    /// IdUsage property
    #[dbus_proxy(property)]
    fn id_usage(&self) -> zbus::Result<String>;

    /// IdVersion property
    #[dbus_proxy(property)]
    fn id_version(&self) -> zbus::Result<String>;

    /// MDRaid property
    #[dbus_proxy(property)]
    fn mdraid(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// MDRaidMember property
    #[dbus_proxy(property)]
    fn mdraid_member(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// PreferredDevice property
    #[dbus_proxy(property)]
    fn preferred_device(&self) -> zbus::Result<AsciiString>;

    /// ReadOnly property
    #[dbus_proxy(property)]
    fn read_only(&self) -> zbus::Result<bool>;

    /// Size property
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;

    /// Symlinks property
    #[dbus_proxy(property)]
    fn symlinks(&self) -> zbus::Result<Vec<AsciiString>>;

    /// UserspaceMountOptions property
    #[dbus_proxy(property)]
    fn userspace_mount_options(&self) -> zbus::Result<Vec<String>>;
}
