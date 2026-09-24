use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use jiff::{Unit, Zoned};
use itertools::Itertools;
use regex::Regex;

static LINE_WIDTH: usize = 40;
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
    // let driver = ConsoleDriver::open(true);
    let driver = UsbDriver::open(0x0519, 0x0001, None, None)?;
    let mut printer = Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));

        printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .page_code(PageCode::PC437)?;
    //    .smoothing(true)?
    //    .bold(true)?
    //    .underline(UnderlineMode::Single)?
    //    .writeln("Bold underline")?
    //    .justify(JustifyMode::CENTER)?
    //    .reverse(true)?
    //    .bold(false)?
    //    .writeln("Hello world - Reverse")?
    //    .feed()?
    //    .justify(JustifyMode::RIGHT)?
    //    .reverse(false)?
    //    .underline(UnderlineMode::None)?
    //    .size(1, 1)?
    //    .writeln("Hello world - Normal─")?;

    print_header(&mut printer);
    print_end(&mut printer);

    Ok(())
}

fn print_scaled_str(p :&mut Printer<UsbDriver>, input: &str, w_scale: u8, h_scale: u8, width: u8, height: u8) -> Result<()> {
    p.size(width*w_scale, height*h_scale)?;
    p.write(input)?;
    p.size(width, height)?;
    Ok(())
}

fn print_line(p: &mut Printer<UsbDriver>, string: &str) {

    let re = Regex::new(r"^[^#|]+|\#[^#|]*").unwrap();

    let split: Vec<&str> = re.find_iter(string)
        .map(|m| m.as_str())
        .collect();

    for i in split {
        if i.contains("#"){ // Escape Character Hit
            let escape_index = i.chars().position(|c| c == '#').unwrap();
            let code = i.chars().nth(escape_index+1).unwrap();
            if code == 'b' {
                p.bold(true);
                p.write(&i[escape_index+1..]).unwrap();
                p.bold(false);
            }
            if code == 'u' {
                p.underline(UnderlineMode::Single);
                p.write(&i[2..]).unwrap();
                p.underline(UnderlineMode::None);
            }
            if code == 'U' {
                p.underline(UnderlineMode::Double);
                p.write(&i[2..]).unwrap();
                p.underline(UnderlineMode::None);
            }
            if code == 'r' {
                p.reverse(true);
                p.write(&i[2..]).unwrap();
                p.reverse(false);
            }
            if code == '4' {
                p.size(1, 4);
                p.write(&i[2..]).unwrap();
                p.reset_size();
            }
        } else { p.write(i); }
    }
}

fn print_in_box(p: &mut Printer<UsbDriver>, content: Vec<&str>, bold: bool, double: bool) -> Result<()> {
    let mut box_char = ["┌","┐","└","┘","─","│"];
    if double {
        box_char = ["╔","╗","╚","╝","═","║"]
    }

    let header_bar = &format!(
        "{}{}{}",
        box_char[0],
        box_char[4].repeat(LINE_WIDTH),
        box_char[1]
    );
    p.writeln(&header_bar)?;

    for x in content { // For every line in content
        if x.len() > LINE_WIDTH-2 { // Check if string would cause overflow. If so, split into chunks and print each chunk.
            let i = x.chars()
                .chunks(LINE_WIDTH)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .collect::<Vec<String>>();
            for j in i {
                print_line(p, &format!("{}{:<width$}{}", box_char[5], j, box_char[5], width = LINE_WIDTH))
            }
        } else {
            print_line(p, &format!("{}{:<width$}{}", box_char[5], x, box_char[5], width = LINE_WIDTH))
        }
    }

    let footer_bar = &format!(
        "{}{}{}",
        box_char[2],
        box_char[4].repeat(LINE_WIDTH),
        box_char[3]
    );
    p.writeln(&footer_bar)?;

    Ok(())

}
fn print_end(p: &mut Printer<UsbDriver>) -> Result<()> {
    p.print_cut()?; // print() or print_cut() is mandatory to send the data to the printer
    Ok(())
}

fn print_header(p: &mut Printer<UsbDriver>) -> Result<()> {
    p.size(1, 2)?;
    let mut content = vec!["#4OVERVW"];
    let rfc_date = &jiff::fmt::rfc2822::to_string(&Zoned::now()).expect("Date Formatting Error");
    content.extend(["", rfc_date, "", "hsdojfghDJIGFHSDIJGHASDFJKGHJKDFAHGJKDAHGKDHGJDSHGJKDFSHGJKLHSKJGAEHKJHAJIHAJHGAEHJHOHOFHHFJOAHSDFGJKHADFJKGHSAJOfghadkjghadjkhgjksdhgjksdhgjka;dhgjkHDGJK;SDHFGJKSHGjHDJKSGHA;KJ"]);
    print_in_box(p,content.clone(), false, false)?;
    print_in_box(p,content.clone(), false, false)?;

    // Construct center Title
    //p.size(1, 2)?;
    //let title = &format!("{:>width$}", title, width = LINE_WIDTH/4);
    //p.write(LIGHT_VERTICAL);

    //p.bold(true)?;
    //print_scaled_str(p, title, 4, 1, 1, 2)?;
    //p.bold(false)?;

    //let mut header_bar = String::from("┌");
    //header_bar.push_str(&LIGHT_HORIZONTAL.repeat(LINE_WIDTH));
    //header_bar.push_str("┐");
    //p.writeln(&header_bar)?;

    Ok(())
}