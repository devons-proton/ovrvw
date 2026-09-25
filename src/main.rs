use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use jiff::{Unit, Zoned};
use itertools::Itertools;
use regex::Regex;
use codepage_437::{BorrowFromCp437, ToCp437, CP437_CONTROL};

static LINE_WIDTH: usize = 40;
static CONTENT_WIDTH: usize = 38;

//static USB_DEVICE: struct usb_device {
//    vendor_id: str = "0x000",
//
//}

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

    let content: Vec<&str> = vec!["OVERVW", " ", "ehruwheruhewurewhoiewroiewhfhdsfhdsjfkdsjflkdsjfkdjkdjlkjdlkfskdljflkdsfdfds"];
    print_box(&mut printer, "TITLE", content);
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

fn print_box(p: &mut Printer<UsbDriver>, heading: &str, content: Vec<&str>){
    static BORDER_CHAR_HEX: [u8; 6] = [0xDA, 0xBF, 0xC0, 0xD9, 0xB3, 0xC4]; // TL, TR, BL, BR, V, H

    println!("{}", heading);
    let mut upbar: Vec<u8> = vec![BORDER_CHAR_HEX[0]];
    upbar.extend(std::iter::repeat(BORDER_CHAR_HEX[5]).take(CONTENT_WIDTH));
    upbar.push(BORDER_CHAR_HEX[1]);
    p.custom(upbar.as_slice().try_into().unwrap());
    print_to_terminal(&upbar);

    for i in content {
        let content_split: Vec<String> = i.chars().chunks(CONTENT_WIDTH).into_iter().map(|c| c.collect::<String>()).collect();

        for j in content_split {
            if j.len() > LINE_WIDTH {
                let content_bytes = j.to_cp437(&CP437_CONTROL).unwrap().into_owned();
                let mut mid_line: Vec<u8> = vec![BORDER_CHAR_HEX[4]];
                mid_line.extend(&content_bytes);
                mid_line.push(BORDER_CHAR_HEX[4]);
                print_to_terminal(&mid_line);
                p.custom(mid_line.as_slice().try_into().unwrap());

            } else {
                let content_bytes = j.to_cp437(&CP437_CONTROL).unwrap().into_owned();
                let mut mid_line: Vec<u8> = vec![BORDER_CHAR_HEX[4]];
                mid_line.extend(&content_bytes);
                mid_line.extend(std::iter::repeat(b' ').take(CONTENT_WIDTH - (j.len())));
                mid_line.push(BORDER_CHAR_HEX[4]);
                p.custom(mid_line.as_slice().try_into().unwrap());
                print_to_terminal(&mid_line);
            }
        }
    }

    let mut downbar: Vec<u8> = vec![BORDER_CHAR_HEX[2]];
    downbar.extend(std::iter::repeat(BORDER_CHAR_HEX[5]).take(CONTENT_WIDTH));
    downbar.push(BORDER_CHAR_HEX[3]);
    p.custom(downbar.as_slice().try_into().unwrap());
    print_to_terminal(&downbar);


}
fn print_to_terminal(vector: &Vec<u8>) { // For debugging
    println!("{}", String::borrow_from_cp437(&vector, &CP437_CONTROL));
}
fn print_end(p: &mut Printer<UsbDriver>) -> Result<()> {
    p.print_cut()?; // print() or print_cut() is mandatory to send the data to the printer
    Ok(())
}