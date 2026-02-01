use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> Result<()> {
    let audio_host = cpal::default_host();

    for id in audio_host.input_devices()? {
        println!("{:?}", id.description()?.extended());
        for sc in id.supported_input_configs()? {
            println!("{sc:?}");
        }
    }

    for id in audio_host.output_devices()? {
        println!("{:?}", id.description()?.extended());
        for sc in id.supported_output_configs()? {
            println!("{sc:?}");
        }
    }

    Ok(())
}
