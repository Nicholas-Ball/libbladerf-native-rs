use tokio::time::sleep;
use libbladerf_native_rs::BladerfDirection::RX;
use libbladerf_native_rs::BladerfVersion;
use libbladerf_native_rs::usb::list_devices;

#[tokio::test]
async fn connect_test() {
    let mut devices = list_devices::<4>().await.unwrap();
    let mut device = devices[0].take().unwrap();

    assert!(!device.is_connected());

    device.connect().await.unwrap();

    assert!(device.is_connected());

    let version = device.get_version().await.unwrap();

    assert_eq!(version, BladerfVersion{major: 2, minor: 4, patch: 0});

    device.disconnect().unwrap();

    assert!(!device.is_connected());
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

    let timestamp = device.get_timestamp(RX).await.unwrap();

    println!("Timestamp: {}", timestamp);

    device.disconnect().unwrap();

    assert!(!device.is_connected());
}