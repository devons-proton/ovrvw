use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use itertools::Itertools;
use codepage_437::{BorrowFromCp437, ToCp437, CP437_CONTROL};
use jiff::{Zoned, Unit};

static LINE_WIDTH: usize = 42;
static CONTENT_WIDTH: usize = LINE_WIDTH - 2;

fn main() -> Result<()> {
    let driver = UsbDriver::open(0x0519, 0x0001, None, None)?;
    let mut printer = Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));

    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .page_code(PageCode::PC437)?
        .smoothing(true)?;

    // Reset to normal standard 1x width, 1x height (or font A)
    printer.size(1, 2)?;

    let now = String::from(Zoned::now().round(Unit::Second).unwrap().to_string());
    let content: Vec<&str> = vec![
        "OVERVW",
        " ",
        now.as_str()
    ];
    print_box(&mut printer, "TITLE", content)?;


    // Feed lines past the print head and trigger the mechanical auto-cutter
    printer.feed()?;
    printer.print_cut()?;

    // Small delay to ensure the OS finishes USB bulk transfer before process teardown
    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(())
}

fn print_box(p: &mut Printer<UsbDriver>, heading: &str, content: Vec<&str>) -> Result<()> {
    static BORDER_CHAR_HEX: [u8; 6] = [0xDA, 0xBF, 0xC0, 0xD9, 0xB3, 0xC4]; // TL, TR, BL, BR, V, H

    p.writeln(heading)?;
    println!("{}", heading);

    // 1. Top Bar
    let mut upbar: Vec<u8> = vec![BORDER_CHAR_HEX[0]];
    upbar.extend(std::iter::repeat(BORDER_CHAR_HEX[5]).take(CONTENT_WIDTH));
    upbar.push(BORDER_CHAR_HEX[1]);
    upbar.push(b'\n'); // Newline needed for the line-buffer

    p.custom(&upbar)?;
    print_to_terminal(&upbar);

    // 2. Middle Content
    for line in content {
        let content_split: Vec<String> = line
            .chars()
            .chunks(CONTENT_WIDTH)
            .into_iter()
            .map(String::from_iter)
            .collect();

        for j in content_split {
            let content_bytes = j.to_cp437(&CP437_CONTROL).unwrap_or_default().into_owned();
            let mut mid_line: Vec<u8> = vec![BORDER_CHAR_HEX[4]];
            mid_line.extend(&content_bytes);

            // Pad to fit exact border width
            let padding = CONTENT_WIDTH.saturating_sub(content_bytes.len());
            if padding > 0 {
                mid_line.extend(std::iter::repeat(b' ').take(padding));
            }

            mid_line.push(BORDER_CHAR_HEX[4]);
            mid_line.push(b'\n');

            p.custom(&mid_line)?;
            print_to_terminal(&mid_line);
        }
    }

    // 3. Bottom Bar
    let mut downbar: Vec<u8> = vec![BORDER_CHAR_HEX[2]];
    downbar.extend(std::iter::repeat(BORDER_CHAR_HEX[5]).take(CONTENT_WIDTH));
    downbar.push(BORDER_CHAR_HEX[3]);
    downbar.push(b'\n');

    p.custom(&downbar)?;
    print_to_terminal(&downbar);

    Ok(())
}

fn print_to_terminal(vector: &[u8]) {
    print!("{}", String::borrow_from_cp437(vector, &CP437_CONTROL));
}