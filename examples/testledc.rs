use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;
use rs_ws281x::ChannelBuilder;
use std::time::Duration;
use std::thread;

const DELAY: Duration = Duration::from_millis(1000);

fn main() {
    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop
    // get the strand of LEDs on channel 1
    // let leds = controller.leds_mut(0);
    // set the first LED to white (with the configured
    // strip above, this is BGRW)
    // leds[0] = [255, 255, 255, 0]
    // render it to the strand
    // controller.render();

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(10) // GPIO 10 = SPI0 MOSI
                .count(4) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(128) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = [255, 0, 0, 0];
    }
    // let leds_clone = leds.clone();
    println!("Blue");
    controller.render().unwrap();
    thread::sleep(DELAY*2);
   
    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 255, 0, 0];
    }
    // let leds_clone = leds.clone();
    println!("Green");
    controller.render().unwrap();
    thread::sleep(DELAY*2);

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 255, 0];
    }
    // let leds_clone = leds.clone();
    println!("Green");
    controller.render().unwrap();
    thread::sleep(DELAY*2);
}

