pub mod ascii;
pub mod ata;
pub mod block;
pub mod drive;
pub mod encryption;
pub mod filesystem;
pub mod job;
pub mod r#loop;
pub mod manager;
pub mod partition;
pub mod raid;
pub mod swap;

use crate::block::BlockProxy;
use crate::manager::UDisks2ManagerProxy;
use zbus::fdo::{ManagedObjects, ObjectManagerProxy};
use zbus::zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};
use zbus::Connection;

use crate::filesystem::FilesystemProxy;

pub struct UDisks2 {
    connection: Connection,
    object_manager: zbus::fdo::ObjectManagerProxy<'static>,
    manager: UDisks2ManagerProxy<'static>,
}

impl UDisks2 {
    pub async fn from_connection(connection: Connection) -> zbus::Result<Self> {
        let om = ObjectManagerProxy::builder(&connection)
            .destination("org.freedesktop.UDisks2")
            .unwrap()
            .path("/org/freedesktop/UDisks2")
            .unwrap()
            .build()
            .await?;

        let manager = UDisks2ManagerProxy::new(&connection).await?;

        Ok(Self {
            connection,
            object_manager: om,
            manager,
        })
    }

    pub async fn lookup_block_devices(
        &self,
        devices: Vec<OwnedObjectPath>,
    ) -> zbus::Result<Vec<BlockDevice>> {
        let mut objects: ManagedObjects = self.object_manager.get_managed_objects().await?;
        let mut results = Vec::new();

        for path in devices {
            if let Some(ifaces) = objects.remove(&path) {
                if !ifaces.contains_key("org.freedesktop.UDisks2.Block") {
                    continue;
                }

                let bproxy = BlockProxy::builder(&self.connection)
                    .path(path.clone())?
                    .build()
                    .await?;

                let fsproxy = if ifaces.contains_key("org.freedesktop.UDisks2.Filesystem") {
                    Some(
                        FilesystemProxy::builder(&self.connection)
                            .path(path.clone())?
                            .build()
                            .await?,
                    )
                } else {
                    None
                };

                results.push(BlockDevice {
                    path,
                    block: bproxy,
                    filesystem: fsproxy,
                });
            }
        }

        Ok(results)
    }

    pub fn manager(&self) -> &UDisks2ManagerProxy<'static> {
        &self.manager
    }
}

pub struct BlockDevice {
    pub path: OwnedObjectPath,
    pub block: BlockProxy<'static>,
    pub filesystem: Option<FilesystemProxy<'static>>,
}

#[derive(DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct StandardOptions {
    #[zvariant(rename = "auth.no_user_interaction")]
    pub no_user_interaction: bool,
}
