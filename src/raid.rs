use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.MDRaid")]
trait MDRaid {
    /// AddDevice method
    fn add_device(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Delete method
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// RemoveDevice method
    fn remove_device(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// RequestSyncAction method
    fn request_sync_action(
        &self,
        sync_action: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetBitmapLocation method
    fn set_bitmap_location(
        &self,
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Start method
    fn start(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Stop method
    fn stop(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ActiveDevices property
    #[dbus_proxy(property)]
    fn active_devices(
        &self,
    ) -> zbus::Result<
        Vec<(
            zbus::zvariant::OwnedObjectPath,
            i32,
            Vec<String>,
            u64,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// BitmapLocation property
    #[dbus_proxy(property)]
    fn bitmap_location(&self) -> zbus::Result<Vec<u8>>;

    /// ChildConfiguration property
    #[dbus_proxy(property)]
    fn child_configuration(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// ChunkSize property
    #[dbus_proxy(property)]
    fn chunk_size(&self) -> zbus::Result<u64>;

    /// Degraded property
    #[dbus_proxy(property)]
    fn degraded(&self) -> zbus::Result<u32>;

    /// Level property
    #[dbus_proxy(property)]
    fn level(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// NumDevices property
    #[dbus_proxy(property)]
    fn num_devices(&self) -> zbus::Result<u32>;

    /// Running property
    #[dbus_proxy(property)]
    fn running(&self) -> zbus::Result<bool>;

    /// Size property
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;

    /// SyncAction property
    #[dbus_proxy(property)]
    fn sync_action(&self) -> zbus::Result<String>;

    /// SyncCompleted property
    #[dbus_proxy(property)]
    fn sync_completed(&self) -> zbus::Result<f64>;

    /// SyncRate property
    #[dbus_proxy(property)]
    fn sync_rate(&self) -> zbus::Result<u64>;

    /// SyncRemainingTime property
    #[dbus_proxy(property)]
    fn sync_remaining_time(&self) -> zbus::Result<u64>;

    /// UUID property
    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;
}
