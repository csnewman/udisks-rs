use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.PartitionTable")]
trait PartitionTable {
    /// CreatePartition method
    fn create_partition(
        &self,
        offset: u64,
        size: u64,
        type_: &str,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CreatePartitionAndFormat method
    fn create_partition_and_format(
        &self,
        offset: u64,
        size: u64,
        type_: &str,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        format_type: &str,
        format_options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Partitions property
    #[dbus_proxy(property)]
    fn partitions(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<String>;
}

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Partition")]
trait Partition {
    /// Delete method
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Resize method
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetFlags method
    fn set_flags(
        &self,
        flags: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetName method
    fn set_name(
        &self,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetType method
    fn set_type(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u64>;

    /// IsContained property
    #[dbus_proxy(property)]
    fn is_contained(&self) -> zbus::Result<bool>;

    /// IsContainer property
    #[dbus_proxy(property)]
    fn is_container(&self) -> zbus::Result<bool>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Number property
    #[dbus_proxy(property)]
    fn number(&self) -> zbus::Result<u32>;

    /// Offset property
    #[dbus_proxy(property)]
    fn offset(&self) -> zbus::Result<u64>;

    /// Size property
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;

    /// Table property
    #[dbus_proxy(property)]
    fn table(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> zbus::Result<String>;

    /// UUID property
    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;
}
