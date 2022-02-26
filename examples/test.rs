use udisks::{StandardOptions, UDisks2};
use zbus::Connection;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;

    let udisks = UDisks2::from_connection(connection).await?;

    let manager = udisks.manager();

    let ver = manager.version().await?;
    println!("ver={}", ver);

    for fs in manager.supported_filesystems().await? {
        println!("supported: {}", fs);
    }

    let device_paths = manager
        .get_block_devices(StandardOptions {
            no_user_interaction: false,
        })
        .await?;

    let devices = udisks.lookup_block_devices(device_paths).await?;

    for device in devices {
        println!("device {:?}", device.path);
        println!("  > id={}", device.block.id().await?);

        if let Some(fs) = &device.filesystem {
            println!("  > Filesystem size={}", fs.size().await?);
        }
    }

    Ok(())
}
