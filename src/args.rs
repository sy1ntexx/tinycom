use clap::Clap;

#[derive(Debug, Clap)]
#[clap(
    author = "sy1ntexx",
    version = "0.1",
    about = "Tiny COM/Serial interation program"
)]
#[clap(setting = clap::AppSettings::ColoredHelp)]
pub struct TinyComArgs {
    #[clap(
        long = "baud-rate",
        short = 'r',
        default_value = "115200",
        about = "Baudrate of the device"
    )]
    pub baud_rate: usize,

    #[clap(
        long = "device",
        short = 'd',
        default_value = "/dev/ttyACM0",
        about = "Specifies the device which represents a COM port"
    )]
    pub device: String,

    #[clap(
        long = "read-only",
        short = 'o',
        about = "Disables ability to send input to port"
    )]
    pub read_only: bool,

    #[clap(
        long = "no-capture",
        short = 'c',
        about = "Do not capture input recieved when reading"
    )]
    pub no_capture: bool,
}
