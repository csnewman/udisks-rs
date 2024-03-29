use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Job")]
trait Job {
    /// Cancel method
    fn cancel(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Completed signal
    #[dbus_proxy(signal)]
    fn completed(&self, success: bool, message: &str) -> zbus::Result<()>;

    /// Bytes property
    #[dbus_proxy(property)]
    fn bytes(&self) -> zbus::Result<u64>;

    /// Cancelable property
    #[dbus_proxy(property)]
    fn cancelable(&self) -> zbus::Result<bool>;

    /// ExpectedEndTime property
    #[dbus_proxy(property)]
    fn expected_end_time(&self) -> zbus::Result<u64>;

    /// Objects property
    #[dbus_proxy(property)]
    fn objects(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Operation property
    #[dbus_proxy(property)]
    fn operation(&self) -> zbus::Result<String>;

    /// Progress property
    #[dbus_proxy(property)]
    fn progress(&self) -> zbus::Result<f64>;

    /// ProgressValid property
    #[dbus_proxy(property)]
    fn progress_valid(&self) -> zbus::Result<bool>;

    /// Rate property
    #[dbus_proxy(property)]
    fn rate(&self) -> zbus::Result<u64>;

    /// StartTime property
    #[dbus_proxy(property)]
    fn start_time(&self) -> zbus::Result<u64>;

    /// StartedByUID property
    #[dbus_proxy(property)]
    fn started_by_uid(&self) -> zbus::Result<u32>;
}
