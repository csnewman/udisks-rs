use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Encrypted")]
trait Encrypted {
    /// ChangePassphrase method
    fn change_passphrase(
        &self,
        passphrase: &str,
        new_passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Lock method
    fn lock(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Resize method
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Unlock method
    fn unlock(
        &self,
        passphrase: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

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

    /// CleartextDevice property
    #[dbus_proxy(property)]
    fn cleartext_device(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// HintEncryptionType property
    #[dbus_proxy(property)]
    fn hint_encryption_type(&self) -> zbus::Result<String>;

    /// MetadataSize property
    #[dbus_proxy(property)]
    fn metadata_size(&self) -> zbus::Result<u64>;
}
