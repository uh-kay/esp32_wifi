use core::convert::TryInto;

use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, log::EspLogger, nvs::EspDefaultNvsPartition, 
};
use esp_idf_svc::wifi::{AuthMethod::WPA2Personal, BlockingWifi, ClientConfiguration, Configuration, EspWifi, PmfConfiguration::NotCapable, ScanMethod::CompleteScan, ScanSortMethod};

use log::info;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    let p = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(p.modem, sysloop.clone(), Some(nvs))?,
        sysloop
    )?;

    connect_wifi(&mut wifi)?;

    info!("Connected");

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    info!("Shutting down in 5s...");

    std::thread::sleep(core::time::Duration::from_secs(5));

    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        auth_method: WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        scan_method: CompleteScan(ScanSortMethod::Security),
        pmf_cfg: NotCapable,
        bssid: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;

    wifi.connect()?;

    wifi.wait_netif_up()?;

    Ok(())
}