use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Swapspace")]
trait Swapspace {
    /// SetLabel method
    fn set_label(
        &self,
        label: &str,
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

    /// Active property
    #[dbus_proxy(property)]
    fn active(&self) -> zbus::Result<bool>;
}
