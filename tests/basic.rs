use tokio::time::sleep;
use libbladerf_native_rs::BladerfDirection::RX;
use libbladerf_native_rs::BladerfVersion;
use libbladerf_native_rs::usb::list_devices;
use libbladerf_native_rs::nios::nios_access;

#[tokio::test]
async fn connect_test() {
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let version = device.get_version().await.unwrap();

    assert_eq!(version, BladerfVersion{major: 2, minor: 4, patch: 0});

    device.disconnect().unwrap();

    assert!(!device.is_connected());
}

#[tokio::test]
async fn define_scittamai(){
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());

    for i in 0..255{
        let found_value = nios_access::nios_8x8_read::<16, 16, 0x82>(&device, 0, i).await.unwrap();
        println!("I poked around at {i} and found a value {found_value}!");
    }

    /*let version = device.get_version().await.unwrap();

    assert_eq!(version, BladerfVersion{major: 2, minor: 4, patch: 0});*/

    /*let ver = nios_get_fpga_version::<16, 16, 0>(&device).await.unwrap();
    let ver_major = ver.major;
    let ver_minor = ver.minor;
    let ver_patch = ver.patch;

    println!("Received feedback with data: major {ver_major}, minor {ver_minor}, patch {ver_patch}.");

    //assert_eq!(version, ver);*/
}

#[tokio::test]
async fn enable_rx_test() {
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());

    device.enable_rx().await.unwrap();
    
    sleep(std::time::Duration::from_secs(10)).await;
    
    device.disable_rx().await.unwrap();

    device.disconnect().unwrap();

    assert!(!device.is_connected());
}

#[tokio::test]
async fn enable_tx_test() {
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());

    device.enable_tx().await.unwrap();

    sleep(std::time::Duration::from_secs(10)).await;

    device.disable_tx().await.unwrap();

    device.disconnect().unwrap();

    assert!(!device.is_connected());
}

#[tokio::test]
async fn get_timestamp_test() {
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());
    
    device.enable_rx().await.unwrap();

    //let timestamp = device.get_timestamp(RX).await.unwrap();

    //println!("Timestamp: {}", timestamp);

    device.disconnect().unwrap();

    assert!(!device.is_connected());
}