#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use embassy_nrf::{interrupt, pac, wdt, wdt::Watchdog, wdt::WatchdogHandle, config::{HfclkSource, LfclkSource}};
//use embedded_alloc::Heap;
use embassy_time::Timer;
use embassy_time::Duration;


use core::mem;
use nrf_softdevice::{raw, Softdevice};


const WDT_TIMEOUT : u32 = 75; // seconds
const WDT_FEED_TIME : u64 = 60; // seconds



use defmt_rtt as _; // global logger
use embassy_nrf as _; // memory layout
use panic_probe as _;



use embedded_alloc::Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}



use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;
use core::sync::atomic::{AtomicU32, Ordering};





#[embassy_executor::main]
async fn main(spawner: Spawner) {

    info!("INIT");


    /* Configure peripherals */
    let mut config = embassy_nrf::config::Config::default();
    config.time_interrupt_priority = Priority::P2;
    config.hfclk_source =  HfclkSource::ExternalXtal;
    config.lfclk_source = LfclkSource::ExternalXtal;

    #[allow(unused)]
    let p = embassy_nrf::init(config);


    // hw specific
    //let mut board = Board::init(p);
    // turn off P0.17 (LED1) with PULL_DOWNs. Necessary for MTK, otherwise the LED will glow
    //board.led1.set_low();

    //Timer::after(Duration::from_millis(1_000_000)).await;

    // Enable SoftDevice
    let sd = nrf_softdevice::Softdevice::enable(&softdevice_config());

   


    
    loop { 
        Timer::after(Duration::from_millis(1_000)).await;
    }
}







/// Returns softdevice config.
pub fn softdevice_config() -> nrf_softdevice::Config {


    let se : &[u8; 4]  = b"SCDH";

    nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
            rc_ctiv: 0,
            rc_temp_ctiv: 0,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_20_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 1,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: 1024,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 1,
            central_role_count: 0,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: se.as_ptr()  as _,
            current_len: 4,
            max_len: 14,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                raw::BLE_GATTS_VLOC_STACK as u8,
            ),
        }),
        ..Default::default()
    }
}




use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    
    //peripherals::{TWISPI0},
    //twim::Twim,
    //uarte::{self, Uarte},
};


pub struct Board {
    /// Onboard LED 1
    pub led1: Output<'static, AnyPin>,
    /// Onboard LED 2
    pub led2: Output<'static, AnyPin>,
    /// Onboard LED 3
    pub led3: Output<'static, AnyPin>,
    /// Onboard LED 4
    pub led4: Output<'static, AnyPin>,
    /// Onboard Button 1
    pub button1: Input<'static, AnyPin>,
    /// Onboard Button 2
    pub button2: Input<'static, AnyPin>,
    /// Onboard Button 3
    pub button3: Input<'static, AnyPin>,
    /// Onboard Button 4
    pub button4: Input<'static, AnyPin>,
    // TWI
    //pub twim: Twim<'static, TWISPI0>,
    // UART: Serial - USB
    //pub uart: Uarte<'static, UARTE0>,
}

impl Board {
    /// Returns Board with concrete peripherals
    pub fn init(p: embassy_nrf::Peripherals) -> Board {
        let led1 = Output::new(p.P0_17.degrade(), Level::High, OutputDrive::Standard);
        let led2 = Output::new(p.P0_18.degrade(), Level::High, OutputDrive::Standard);
        let led3 = Output::new(p.P0_19.degrade(), Level::High, OutputDrive::Standard);
        let led4 = Output::new(p.P0_20.degrade(), Level::High, OutputDrive::Standard);

        let button1 = Input::new(p.P0_13.degrade(), Pull::Up);
        let button2 = Input::new(p.P0_14.degrade(), Pull::Up);
        let button3 = Input::new(p.P0_15.degrade(), Pull::Up);
        let button4 = Input::new(p.P0_16.degrade(), Pull::Up);

        // configure twi
        //let twim_config = embassy_nrf::twim::Config::default();
        //let twim_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
        //twim_irq.set_priority(Priority::P3);
        //let twim = Twim::new(p.TWISPI0, twim_irq, p.P0_26, p.P0_27, twim_config);

        // configure uart
        /*
        let mut uart_config = uarte::Config::default();
        uart_config.parity = uarte::Parity::EXCLUDED;
        uart_config.baudrate = uarte::Baudrate::BAUD115200;
        let uart_irq = interrupt::take!(UARTE0_UART0);
        uart_irq.set_priority(Priority::P3);
        let uart = uarte::Uarte::new(p.UARTE0, uart_irq, p.P0_08, p.P0_06, uart_config);
        */
        Board {
            led1,
            led2,
            led3,
            led4,
            button1,
            button2,
            button3,
            button4,
            //twim,
            //uart,
        }
    }
}