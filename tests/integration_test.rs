extern crate sonos;

use sonos::TransportState;

fn get_speaker() -> sonos::Speaker {
    let devices = sonos::discover().unwrap();

    devices
        .into_iter()
        .find(|d| d.name == "Living Room")
        .ok_or("Couldn't find bedroom")
        .unwrap()
}

#[test]
fn can_discover_devices() {
    let devices = sonos::discover().unwrap();
    assert!(!devices.is_empty(), "No devices discovered");
}

#[test]
fn volume() {
    let device = get_speaker();
    device.set_volume(2).expect("Failed to get volume");
    assert_eq!(
        device.volume().expect("Failed to get volume"),
        2 as u8,
        "Volume was not updated."
    );
}

#[test]
fn muted() {
    let device = get_speaker();
    device.mute().expect("Couldn't mute player");
    assert_eq!(
        device.muted().expect("Failed to get current mute status"),
        true
    );
    device.unmute().expect("Couldn't unmute player");
    assert_eq!(
        device.muted().expect("Failed to get current mute status"),
        false
    );
}

#[test]
fn playback_state() {
    let device = get_speaker();

    device.play().expect("Couldn't play track");
    assert!(match device.transport_state().unwrap() {
        TransportState::Playing | TransportState::Transitioning => true,
        _ => false,
    });

    device.pause().expect("Couldn't pause track");
    assert!(match device.transport_state().unwrap() {
        TransportState::PausedPlayback | TransportState::Transitioning => true,
        _ => false,
    });

    device.stop().expect("Couldn't stop track");
    assert!(match device.transport_state().unwrap() {
        TransportState::Stopped | TransportState::Transitioning => true,
        _ => false,
    });
}

#[test]
fn track_info() {
    let device = get_speaker();
    device.track().expect("Failed to get track info");
}

#[test]
fn seek() {
    let device = get_speaker();
    device
        .seek(&std::time::Duration::from_secs(30))
        .expect("Failed to seek to 30 seconds");
    assert_eq!(
        device
            .track()
            .expect("Failed to get track info")
            .running_time
            .as_secs(),
        30
    );
}

#[test]
fn play() {
    let device = get_speaker();
    device.play().expect("Failed to play");
    device.pause().expect("Failed to pause");
}

#[test]
#[should_panic]
fn fail_on_set_invalid_volume() {
    get_speaker()
        .set_volume(101)
        .expect_err("Didn't fail on invalid volume");
}
