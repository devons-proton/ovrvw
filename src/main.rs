use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use jiff::{Zoned};

static LINE_WIDTH: usize = 42;
static LIGHT_HORIZONTAL: &str = "─";
static LIGHT_VERTICAL: &str = "│";
static TITLE: &str = "OVERVW";

//static USB_DEVICE: struct usb_device {
//    vendor_id: str = "0x000",
//
//}

static NORMAL_TEXT_WIDTH: usize = 2;
static NORMAL_TEXT_HEIGHT: usize = 3;

static TITLE_TEXT_WIDTH: usize = 4;
static TITLE_TEXT_HEIGHT: usize = 5;

fn main() -> Result<()> {
    // env_logger::init();

    // let driver = NetworkDriver::open("192.168.1.248", 9100, None)?;
    let driver = ConsoleDriver::open(true);
    let mut printer = Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));

        printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
        .bold(true)?
        .underline(UnderlineMode::Single)?
        .writeln("Bold underline")?
        .justify(JustifyMode::CENTER)?
        .reverse(true)?
        .bold(false)?
        .writeln("Hello world - Reverse")?
        .feed()?
        .justify(JustifyMode::RIGHT)?
        .reverse(false)?
        .underline(UnderlineMode::None)?
        .size(2, 3)?
        .writeln("Hello world - Normal")?;

    print_header(&mut printer, TITLE);
    print_end(&mut printer);

    Ok(())
}

fn print_end(p: &mut Printer<ConsoleDriver>) -> Result<()> {
    p.print_cut()?; // print() or print_cut() is mandatory to send the data to the printer
    Ok(())
}

fn print_header(p: &mut Printer<ConsoleDriver>, title: &str) -> Result<()> {
    p.justify(JustifyMode::CENTER)?;
    p.size(2, 3)?;

    // Construct, dynamically header_bar to LINE_WIDTH, and print.
    let mut header_bar = String::from("┌");
    header_bar.push_str(&LIGHT_HORIZONTAL.repeat(LINE_WIDTH));
    header_bar.push_str("┐");
    p.writeln(&header_bar)?;

    // Construct center Title
    let mut title_bar = String::from("│");
    p.writeln(&format!("{LIGHT_VERTICAL}{:^width$}{LIGHT_VERTICAL}", title, width = LINE_WIDTH))?;
    p.writeln(&format!("{LIGHT_VERTICAL}{:^width$}{LIGHT_VERTICAL}", Zoned::now().datetime(), width = LINE_WIDTH))?;

    Ok(())
}