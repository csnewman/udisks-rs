use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Drive")]
trait Drive {
    /// Eject method
    fn eject(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// PowerOff method
    fn power_off(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetConfiguration method
    fn set_configuration(
        &self,
        value: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// CanPowerOff property
    #[dbus_proxy(property)]
    fn can_power_off(&self) -> zbus::Result<bool>;

    /// Configuration property
    #[dbus_proxy(property)]
    fn configuration(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ConnectionBus property
    #[dbus_proxy(property)]
    fn connection_bus(&self) -> zbus::Result<String>;

    /// Ejectable property
    #[dbus_proxy(property)]
    fn ejectable(&self) -> zbus::Result<bool>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// Media property
    #[dbus_proxy(property)]
    fn media(&self) -> zbus::Result<String>;

    /// MediaAvailable property
    #[dbus_proxy(property)]
    fn media_available(&self) -> zbus::Result<bool>;

    /// MediaChangeDetected property
    #[dbus_proxy(property)]
    fn media_change_detected(&self) -> zbus::Result<bool>;

    /// MediaCompatibility property
    #[dbus_proxy(property)]
    fn media_compatibility(&self) -> zbus::Result<Vec<String>>;

    /// MediaRemovable property
    #[dbus_proxy(property)]
    fn media_removable(&self) -> zbus::Result<bool>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    /// Optical property
    #[dbus_proxy(property)]
    fn optical(&self) -> zbus::Result<bool>;

    /// OpticalBlank property
    #[dbus_proxy(property)]
    fn optical_blank(&self) -> zbus::Result<bool>;

    /// OpticalNumAudioTracks property
    #[dbus_proxy(property)]
    fn optical_num_audio_tracks(&self) -> zbus::Result<u32>;

    /// OpticalNumDataTracks property
    #[dbus_proxy(property)]
    fn optical_num_data_tracks(&self) -> zbus::Result<u32>;

    /// OpticalNumSessions property
    #[dbus_proxy(property)]
    fn optical_num_sessions(&self) -> zbus::Result<u32>;

    /// OpticalNumTracks property
    #[dbus_proxy(property)]
    fn optical_num_tracks(&self) -> zbus::Result<u32>;

    /// Removable property
    #[dbus_proxy(property)]
    fn removable(&self) -> zbus::Result<bool>;

    /// Revision property
    #[dbus_proxy(property)]
    fn revision(&self) -> zbus::Result<String>;

    /// RotationRate property
    #[dbus_proxy(property)]
    fn rotation_rate(&self) -> zbus::Result<i32>;

    /// Seat property
    #[dbus_proxy(property)]
    fn seat(&self) -> zbus::Result<String>;

    /// Serial property
    #[dbus_proxy(property)]
    fn serial(&self) -> zbus::Result<String>;

    /// SiblingId property
    #[dbus_proxy(property)]
    fn sibling_id(&self) -> zbus::Result<String>;

    /// Size property
    #[dbus_proxy(property)]
    fn size(&self) -> zbus::Result<u64>;

    /// SortKey property
    #[dbus_proxy(property)]
    fn sort_key(&self) -> zbus::Result<String>;

    /// TimeDetected property
    #[dbus_proxy(property)]
    fn time_detected(&self) -> zbus::Result<u64>;

    /// TimeMediaDetected property
    #[dbus_proxy(property)]
    fn time_media_detected(&self) -> zbus::Result<u64>;

    /// Vendor property
    #[dbus_proxy(property)]
    fn vendor(&self) -> zbus::Result<String>;

    /// WWN property
    #[dbus_proxy(property)]
    fn wwn(&self) -> zbus::Result<String>;
}
