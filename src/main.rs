extern crate core;

use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_svc::wifi::*;
use embedded_svc::wifi::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::netif::{EspNetifStack};
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use embedded_hal::digital::blocking::OutputPin;
use esp_idf_sys::*;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    link_patches();

    const SSID: &str = "AP-test";
    const PASS: &str = "12345678";

    #[allow(unused)]
        let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    #[allow(unused)]
        let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    #[allow(unused)]
        let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs).unwrap());

    let mut led = Peripherals::take().unwrap().pins.gpio2.into_output().unwrap();


    wifi.set_configuration(&Configuration::AccessPoint(
        AccessPointConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            ..Default::default()
        },
    )).unwrap();

    let wifi_sta_list =  &mut wifi_sta_list_t  {
        ..Default::default()
    } as *mut wifi_sta_list_t;


    loop {
        unsafe {
            esp_wifi_ap_get_sta_list(wifi_sta_list);
            if (*wifi_sta_list).num > 0 {led.set_high()} else {led.set_low()};
            sleep(Duration::from_millis(500));
        }
    }
}

