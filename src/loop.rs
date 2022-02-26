use zbus::dbus_proxy;

use crate::ascii::AsciiString;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Loop")]
trait Loop {
    /// Delete method
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetAutoclear method
    fn set_autoclear(
        &self,
        value: bool,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Autoclear property
    #[dbus_proxy(property)]
    fn autoclear(&self) -> zbus::Result<bool>;

    /// BackingFile property
    #[dbus_proxy(property)]
    fn backing_file(&self) -> zbus::Result<AsciiString>;

    /// SetupByUID property
    #[dbus_proxy(property)]
    fn setup_by_uid(&self) -> zbus::Result<u32>;
}
