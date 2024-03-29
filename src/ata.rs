use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.UDisks2.Drive.Ata")]
trait Ata {
    /// PmGetState method
    fn pm_get_state(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<u8>;

    /// PmStandby method
    fn pm_standby(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// PmWakeup method
    fn pm_wakeup(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SecurityEraseUnit method
    fn security_erase_unit(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SmartGetAttributes method
    fn smart_get_attributes(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<
        Vec<(
            u8,
            String,
            u16,
            i32,
            i32,
            i32,
            i64,
            i32,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// SmartSelftestAbort method
    fn smart_selftest_abort(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SmartSelftestStart method
    fn smart_selftest_start(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SmartSetEnabled method
    fn smart_set_enabled(
        &self,
        value: bool,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SmartUpdate method
    fn smart_update(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// AamEnabled property
    #[dbus_proxy(property)]
    fn aam_enabled(&self) -> zbus::Result<bool>;

    /// AamSupported property
    #[dbus_proxy(property)]
    fn aam_supported(&self) -> zbus::Result<bool>;

    /// AamVendorRecommendedValue property
    #[dbus_proxy(property)]
    fn aam_vendor_recommended_value(&self) -> zbus::Result<i32>;

    /// ApmEnabled property
    #[dbus_proxy(property)]
    fn apm_enabled(&self) -> zbus::Result<bool>;

    /// ApmSupported property
    #[dbus_proxy(property)]
    fn apm_supported(&self) -> zbus::Result<bool>;

    /// PmEnabled property
    #[dbus_proxy(property)]
    fn pm_enabled(&self) -> zbus::Result<bool>;

    /// PmSupported property
    #[dbus_proxy(property)]
    fn pm_supported(&self) -> zbus::Result<bool>;

    /// ReadLookaheadEnabled property
    #[dbus_proxy(property)]
    fn read_lookahead_enabled(&self) -> zbus::Result<bool>;

    /// ReadLookaheadSupported property
    #[dbus_proxy(property)]
    fn read_lookahead_supported(&self) -> zbus::Result<bool>;

    /// SecurityEnhancedEraseUnitMinutes property
    #[dbus_proxy(property)]
    fn security_enhanced_erase_unit_minutes(&self) -> zbus::Result<i32>;

    /// SecurityEraseUnitMinutes property
    #[dbus_proxy(property)]
    fn security_erase_unit_minutes(&self) -> zbus::Result<i32>;

    /// SecurityFrozen property
    #[dbus_proxy(property)]
    fn security_frozen(&self) -> zbus::Result<bool>;

    /// SmartEnabled property
    #[dbus_proxy(property)]
    fn smart_enabled(&self) -> zbus::Result<bool>;

    /// SmartFailing property
    #[dbus_proxy(property)]
    fn smart_failing(&self) -> zbus::Result<bool>;

    /// SmartNumAttributesFailedInThePast property
    #[dbus_proxy(property)]
    fn smart_num_attributes_failed_in_the_past(&self) -> zbus::Result<i32>;

    /// SmartNumAttributesFailing property
    #[dbus_proxy(property)]
    fn smart_num_attributes_failing(&self) -> zbus::Result<i32>;

    /// SmartNumBadSectors property
    #[dbus_proxy(property)]
    fn smart_num_bad_sectors(&self) -> zbus::Result<i64>;

    /// SmartPowerOnSeconds property
    #[dbus_proxy(property)]
    fn smart_power_on_seconds(&self) -> zbus::Result<u64>;

    /// SmartSelftestPercentRemaining property
    #[dbus_proxy(property)]
    fn smart_selftest_percent_remaining(&self) -> zbus::Result<i32>;

    /// SmartSelftestStatus property
    #[dbus_proxy(property)]
    fn smart_selftest_status(&self) -> zbus::Result<String>;

    /// SmartSupported property
    #[dbus_proxy(property)]
    fn smart_supported(&self) -> zbus::Result<bool>;

    /// SmartTemperature property
    #[dbus_proxy(property)]
    fn smart_temperature(&self) -> zbus::Result<f64>;

    /// SmartUpdated property
    #[dbus_proxy(property)]
    fn smart_updated(&self) -> zbus::Result<u64>;

    /// WriteCacheEnabled property
    #[dbus_proxy(property)]
    fn write_cache_enabled(&self) -> zbus::Result<bool>;

    /// WriteCacheSupported property
    #[dbus_proxy(property)]
    fn write_cache_supported(&self) -> zbus::Result<bool>;
}
