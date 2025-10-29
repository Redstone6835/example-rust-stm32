#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 获取外设访问接口
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    // 配置时钟
    let mut rcc = dp.RCC.constrain();
    let mut delay = cp.SYST.delay(&rcc.clocks);
    // GPIOC 时钟开启
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    // PB5 连接板载 LED
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    loop {
        led.toggle(); // 翻转电平
        delay.delay_ms(1000_u32);
    }
}
