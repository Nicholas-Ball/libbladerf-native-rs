use libbladerf_native_rs::list_devices;

#[tokio::test]
async fn connect_test() {
    let mut devices = list_devices().unwrap();
    let mut device = devices.pop().unwrap();

    assert_eq!(device.is_connected(), false);

    device.connect().await.unwrap();

    assert_eq!(device.is_connected(), true);

    let version = device.get_version().await.unwrap();

    assert_eq!(version, [2, 0, 4, 0]);

    device.disconnect().unwrap();

    assert_eq!(device.is_connected(), false);
}